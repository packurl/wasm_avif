const url=new URL('avif.wasm',import.meta.url);
await (await fetch(url)).arrayBuffer();
const src=()=>`(async()=>{
  const mod=await WebAssembly.compileStreaming(await fetch('${url}',{cache:'force-cache'}));
  const imports={
    wbg:{
      __wbg_log_d5989ac76b02a61f: (p,n)=>{
        console.log(
          new TextDecoder().decode(new Uint8Array(wasm.memory.buffer).subarray(p,p+n))
        )
      }
    }
  };
  const wasm=(await WebAssembly.instantiate(mod,imports)).exports;
  const malloc=wasm.__wbindgen_malloc;const free=wasm.__wbindgen_free;
  const fn=({data,width,height,hdr,quality,speed})=>{
    try{
      const r=wasm.__wbindgen_add_to_stack_pointer(-16);
      const n1=data.length;const p1=malloc(n1);
      new Uint8Array(wasm.memory.buffer).set(data,p1);
      wasm.avif_from_imagedata(r,p1,n1,width,height,hdr,quality,speed);
      const arr=new Int32Array(wasm.memory.buffer);
      const p2=arr[r/4];const n2=arr[r/4+1];
      const res=new Uint8Array(wasm.memory.buffer).subarray(p2,p2+n2).slice();
      free(p2,n2);
      return res;
    }finally{
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  };
  onmessage=async msg=>postMessage(fn(msg.data));
  postMessage('ready');
})();`
const worker=await new Promise(r=>{
  const worker=new Worker(URL.createObjectURL(new Blob([src()],{type:'application/javascript'})),{type:'module'});
  worker.onmessage=msg=>{
    if(msg.data==='ready'){
      worker.onmessage=null;
      r(worker);
    }
  };
});
/**
 * Brotli-Decompresses the supplied data.
 * @param {Uint8Array} data
 * @param {number} width
 * @param {number} height
 * @param {boolean} hdr
 * @param {number} quality
 * @param {number} speed
 * @return {Promise<Uint8Array>}
 */
const avif=(data,width,height, hdr=false, quality=50,speed=6)=>new Promise(r=>{
  worker.onmessage=msg=>{
    worker.onmessage=null;
    r(msg.data,width,height,hdr,quality,speed);
  }
  worker.postMessage({data,width,height,hdr,quality,speed});
});

export {
  avif
};
