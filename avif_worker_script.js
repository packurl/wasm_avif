importScripts('./avif_for_importScripts.js');
(async()=>{
  const fn=await avif;
  onmessage=async({data:{bytes,width,height,quality,speed}})=>{
    const res=fn(bytes,width,height,quality,speed);
    postMessage(res,[res.buffer]);
  }
  postMessage('ready');
})();