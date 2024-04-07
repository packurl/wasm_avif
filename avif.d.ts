/* tslint:disable */
/* eslint-disable */
declare module 'avif' {
  /**
   * Encodes the supplied ImageData rgba array.
   * @param {Uint8Array} bytes
   * @param {number} width
   * @param {number} height
   * @param {number} [quality=50] (1 to 100)
   * @param {number} [speed=6] (1 to 10)
   * @return {Uint8Array}
   */
  export function avif(bytes: Uint8Array, width: number, height: number, quality: number, speed: number): Uint8Array;

  export default avif;
}
