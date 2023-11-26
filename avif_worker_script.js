importScripts('./avif_for_importScripts.js');
(async()=>{
  const fn=await avif;
  onmessage=async msg=>{
    postMessage(fn(msg.data.bytes,msg.data.width,msg.data.height,msg.data.quality,msg.data.speed));
  }
  postMessage('ready');
})();