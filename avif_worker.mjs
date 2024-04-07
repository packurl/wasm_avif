const url=new URL('avif.wasm',import.meta.url);
await (await fetch(url)).arrayBuffer();
const worker=await new Promise(r=>{
  // For browsers that don't support type: module on workers (firefox < 114, safari < 15)
  // const worker=new Worker(new URL('./avif_worker_script.js',import.meta.url));
  const worker=new Worker(new URL('./avif_worker_script.mjs',import.meta.url),{type: 'module'});
  worker.onmessage=msg=>{
    if(msg.data==='ready'){
      worker.onmessage=null;
      r(worker);
    }
  };
});
/**
 * Encodes the supplied ImageData rgba array.
 * @param {Uint8Array} bytes
 * @param {number} width
 * @param {number} height
 * @param {number} quality (1 to 100)
 * @param {number} speed (1 to 10)
 * @return {Promise<Uint8Array>}
 */
const avif=(bytes,width,height,quality=50,speed=6)=>new Promise(r=>{
  worker.onmessage=msg=>{
    worker.onmessage=null;
    r(msg.data);
  }
  worker.postMessage({bytes,width,height,quality,speed});
});

export {avif};
export default avif;
