use crate::error::Error;
use rav1e::prelude::*;

pub struct Encoder {
    // 0-255
    quantizer: u8,
    // rav1e preset 1 (slow) 10 (fast but crappy)
    speed: u8,
}

impl Encoder {
    // Quality `1..=100`. Panics if out of range.
    pub fn with_quality(mut self, quality: f32) -> Self {
        debug_assert!((1. ..=100.).contains(&quality));
        self.quantizer = quality_to_quantizer(quality);
        self
    }

    // `1..=10`. 1 = very very slow, but max compression.
    // 10 = quick, but larger file sizes and lower quality.
    pub fn with_speed(mut self, speed: u8) -> Self {
        debug_assert!((1..=10).contains(&speed));
        self.speed = speed;
        self
    }
}

impl Default for Encoder {
    fn default() -> Self {
        Self {
            quantizer: quality_to_quantizer(80.),
            speed: 5,
        }
    }
}

impl Encoder {
    pub fn encode_rgb<'a>(
        &'a self,
        width: usize,
        height: usize,
        pixels: impl Iterator<Item = &'a [u8]> + Send + Sync,
    ) -> Result<Vec<u8>, Error> {
        let planes = pixels.map(|px| {
            let (y, u, v) = rgb_to_ycbcr(px, BT601);
            [y, u, v]
        });
        self.encode_raw_planes(
            width,
            height,
            planes,
            PixelRange::Full,
            MatrixCoefficients::BT601,
        )
    }

    fn encode_raw_planes(
        &self,
        width: usize,
        height: usize,
        planes: impl IntoIterator<Item = [u8; 3]> + Send,
        color_pixel_range: PixelRange,
        matrix_coefficients: MatrixCoefficients,
    ) -> Result<Vec<u8>, Error> {
        let color_description = Some(ColorDescription {
            transfer_characteristics: TransferCharacteristics::SRGB,
            color_primaries: ColorPrimaries::BT709, // sRGB-compatible
            matrix_coefficients,
        });

        let encode_color = move || {
            encode_to_av1(
                &Av1EncodeConfig {
                    width,
                    height,
                    quantizer: self.quantizer.into(),
                    speed: SpeedTweaks::from_my_preset(self.speed, self.quantizer),
                    pixel_range: color_pixel_range,
                    chroma_sampling: ChromaSampling::Cs444,
                    color_description,
                },
                move |frame| init_frame_3(width, height, planes, frame),
            )
        };
        let color = encode_color()?;
        let avif_file = avif_serialize::Aviffy::new()
            .matrix_coefficients(match matrix_coefficients {
                MatrixCoefficients::BT601 => avif_serialize::constants::MatrixCoefficients::Bt601,
                _ => {
                    return Err(Error::Unsupported("matrix coefficients"));
                }
            })
            .to_vec(&color, None, width as u32, height as u32, 8);
        Ok(avif_file)
    }
}

const BT601: [f32; 3] = [0.2990, 0.5870, 0.1140];

fn rgb_to_ycbcr(px: &[u8], matrix: [f32; 3]) -> (u8, u8, u8) {
    let r = f32::from(px[0]);
    let g = f32::from(px[1]);
    let b = f32::from(px[2]);
    let max_value = ((1 << 8) - 1) as f32;
    let scale = max_value / 255.;
    let shift = (max_value * 0.5).round();
    let y = scale * matrix[0] * r + scale * matrix[1] * g + scale * matrix[2] * b;
    let cb = (b * scale - y).mul_add(0.5 / (1. - matrix[2]), shift);
    let cr = (r * scale - y).mul_add(0.5 / (1. - matrix[0]), shift);
    (y.round() as u8, cb.round() as u8, cr.round() as u8)
}

fn quality_to_quantizer(quality: f32) -> u8 {
    let q = quality / 100.;
    let x = if q >= 0.85 {
        (1. - q) * 3.
    } else if q > 0.25 {
        1. - 0.125 - q * 0.5
    } else {
        1. - q
    };
    (x * 255.).round() as u8
}

#[derive(Debug, Copy, Clone)]
struct SpeedTweaks {
    pub speed_preset: u8,

    pub fast_deblock: Option<bool>,
    pub reduced_tx_set: Option<bool>,
    pub tx_domain_distortion: Option<bool>,
    pub tx_domain_rate: Option<bool>,
    pub encode_bottomup: Option<bool>,
    pub rdo_tx_decision: Option<bool>,
    pub cdef: Option<bool>,
    /// loop restoration filter
    pub lrf: Option<bool>,
    pub sgr_complexity_full: Option<bool>,
    pub use_satd_subpel: Option<bool>,
    pub inter_tx_split: Option<bool>,
    pub fine_directional_intra: Option<bool>,
    pub complex_prediction_modes: Option<bool>,
    pub partition_range: Option<(u8, u8)>,
    pub min_tile_size: u16,
}

impl SpeedTweaks {
    pub fn from_my_preset(speed: u8, quantizer: u8) -> Self {
        let low_quality = quantizer < quality_to_quantizer(55.);
        let high_quality = quantizer > quality_to_quantizer(80.);
        let max_block_size = if high_quality { 16 } else { 64 };

        Self {
            speed_preset: speed,

            partition_range: Some(match speed {
                0 => (4, 64.min(max_block_size)),
                1 if low_quality => (4, 64.min(max_block_size)),
                2 if low_quality => (4, 32.min(max_block_size)),
                1..=4 => (4, 16),
                5..=8 => (8, 16),
                _ => (16, 16),
            }),

            complex_prediction_modes: Some(speed <= 1), // 2x-3x slower, 2% better
            sgr_complexity_full: Some(speed <= 2), // 15% slower, barely improves anything -/+1%

            encode_bottomup: Some(speed <= 2), // may be costly (+60%), may even backfire

            // big blocks disabled at 3

            // these two are together?
            rdo_tx_decision: Some(speed <= 4 && !high_quality), // it tends to blur subtle textures
            reduced_tx_set: Some(speed == 4 || speed >= 9), // It interacts with tx_domain_distortion too?

            // 4px blocks disabled at 5
            fine_directional_intra: Some(speed <= 6),
            fast_deblock: Some(speed >= 7 && !high_quality), // mixed bag?

            // 8px blocks disabled at 8
            lrf: Some(low_quality && speed <= 8), // hardly any help for hi-q images. recovers some q at low quality
            cdef: Some(low_quality && speed <= 9), // hardly any help for hi-q images. recovers some q at low quality

            inter_tx_split: Some(speed >= 9), // mixed bag even when it works, and it backfires if not used together with reduced_tx_set
            tx_domain_rate: Some(speed >= 10), // 20% faster, but also 10% larger files!

            tx_domain_distortion: None, // very mixed bag, sometimes helps speed sometimes it doesn't
            use_satd_subpel: Some(false), // doesn't make sense
            min_tile_size: match speed {
                0 => 4096,
                1 => 2048,
                2 => 1024,
                3 => 512,
                4 => 256,
                _ => 128,
            } * if high_quality { 2 } else { 1 },
        }
    }

    pub(crate) fn speed_settings(&self) -> SpeedSettings {
        let mut speed_settings = SpeedSettings::from_preset(self.speed_preset);

        speed_settings.multiref = false;
        speed_settings.rdo_lookahead_frames = 1;
        speed_settings.scene_detection_mode = SceneDetectionSpeed::None;
        speed_settings.motion.include_near_mvs = false;

        if let Some(v) = self.fast_deblock {
            speed_settings.fast_deblock = v;
        }
        if let Some(v) = self.reduced_tx_set {
            speed_settings.transform.reduced_tx_set = v;
        }
        if let Some(v) = self.tx_domain_distortion {
            speed_settings.transform.tx_domain_distortion = v;
        }
        if let Some(v) = self.tx_domain_rate {
            speed_settings.transform.tx_domain_rate = v;
        }
        if let Some(v) = self.encode_bottomup {
            speed_settings.partition.encode_bottomup = v;
        }
        if let Some(v) = self.rdo_tx_decision {
            speed_settings.transform.rdo_tx_decision = v;
        }
        if let Some(v) = self.cdef {
            speed_settings.cdef = v;
        }
        if let Some(v) = self.lrf {
            speed_settings.lrf = v;
        }
        if let Some(v) = self.inter_tx_split {
            speed_settings.transform.enable_inter_tx_split = v;
        }
        if let Some(v) = self.sgr_complexity_full {
            speed_settings.sgr_complexity = if v {
                SGRComplexityLevel::Full
            } else {
                SGRComplexityLevel::Reduced
            }
        };
        if let Some(v) = self.use_satd_subpel {
            speed_settings.motion.use_satd_subpel = v;
        }
        if let Some(v) = self.fine_directional_intra {
            speed_settings.prediction.fine_directional_intra = v;
        }
        if let Some(v) = self.complex_prediction_modes {
            speed_settings.prediction.prediction_modes = if v {
                PredictionModesSetting::ComplexAll
            } else {
                PredictionModesSetting::Simple
            }
        };
        if let Some((min, max)) = self.partition_range {
            debug_assert!(min <= max);
            fn sz(s: u8) -> BlockSize {
                match s {
                    4 => BlockSize::BLOCK_4X4,
                    8 => BlockSize::BLOCK_8X8,
                    16 => BlockSize::BLOCK_16X16,
                    32 => BlockSize::BLOCK_32X32,
                    64 => BlockSize::BLOCK_64X64,
                    128 => BlockSize::BLOCK_128X128,
                    _ => panic!("bad size {s}"),
                }
            }
            speed_settings.partition.partition_range = PartitionRange::new(sz(min), sz(max));
        }

        speed_settings
    }
}

struct Av1EncodeConfig {
    pub width: usize,
    pub height: usize,
    pub quantizer: usize,
    pub speed: SpeedTweaks,
    pub pixel_range: PixelRange,
    pub chroma_sampling: ChromaSampling,
    pub color_description: Option<ColorDescription>,
}

fn rav1e_config(p: &Av1EncodeConfig) -> Config {
    // AV1 needs all the CPU power you can give it,
    // except when it'd create inefficiently tiny tiles
    let tiles = {
        let threads = 1;
        threads.min((p.width * p.height) / (p.speed.min_tile_size as usize).pow(2))
    };
    let speed_settings = p.speed.speed_settings();
    Config::new().with_encoder_config(EncoderConfig {
        width: p.width,
        height: p.height,
        time_base: Rational::new(1, 1),
        sample_aspect_ratio: Rational::new(1, 1),
        bit_depth: 8,
        chroma_sampling: p.chroma_sampling,
        chroma_sample_position: ChromaSamplePosition::Unknown,
        pixel_range: p.pixel_range,
        color_description: p.color_description,
        mastering_display: None,
        content_light: None,
        enable_timing_info: false,
        still_picture: true,
        error_resilient: false,
        switch_frame_interval: 0,
        min_key_frame_interval: 0,
        max_key_frame_interval: 0,
        reservoir_frame_delay: None,
        low_latency: false,
        quantizer: p.quantizer,
        min_quantizer: p.quantizer as _,
        bitrate: 0,
        tune: Tune::Psychovisual,
        tile_cols: 0,
        tile_rows: 0,
        tiles,
        film_grain_params: None,
        level_idx: None,
        speed_settings,
    })
}

fn init_frame_3(
    width: usize,
    height: usize,
    planes: impl IntoIterator<Item = [u8; 3]> + Send,
    frame: &mut Frame<u8>,
) -> Result<(), Error> {
    let mut f = frame.planes.iter_mut();
    let mut planes = planes.into_iter();

    // it doesn't seem to be necessary to fill padding area
    let mut y = f.next().unwrap().mut_slice(Default::default());
    let mut u = f.next().unwrap().mut_slice(Default::default());
    let mut v = f.next().unwrap().mut_slice(Default::default());

    for ((y, u), v) in y
        .rows_iter_mut()
        .zip(u.rows_iter_mut())
        .zip(v.rows_iter_mut())
        .take(height)
    {
        let y = &mut y[..width];
        let u = &mut u[..width];
        let v = &mut v[..width];
        for ((y, u), v) in y.iter_mut().zip(u).zip(v) {
            let px = planes.next().ok_or(Error::TooFewPixels)?;
            *y = px[0];
            *u = px[1];
            *v = px[2];
        }
    }
    Ok(())
}

fn encode_to_av1(
    p: &Av1EncodeConfig,
    init: impl FnOnce(&mut Frame<u8>) -> Result<(), Error>,
) -> Result<Vec<u8>, Error> {
    let mut ctx: Context<u8> = rav1e_config(p)
        .new_context()
        .map_err(|e| Error::Encoding(format!("Invalid Config: {e}")))?;
    let mut frame = ctx.new_frame();

    init(&mut frame)?;
    ctx.send_frame(frame)
        .map_err(|e| Error::Encoding(format!("Send Frame: {e}")))?;
    ctx.flush();

    let mut out = Vec::new();
    loop {
        match ctx.receive_packet() {
            Ok(mut packet) => match packet.frame_type {
                FrameType::KEY => {
                    out.append(&mut packet.data);
                }
                _ => continue,
            },
            Err(EncoderStatus::Encoded) | Err(EncoderStatus::LimitReached) => break,
            Err(err) => Err(err).map_err(|e| Error::Encoding(format!("Receive Packet: {e}")))?,
        }
    }
    Ok(out)
}
