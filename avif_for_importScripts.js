const avif=(async()=>{
  const imports={
    wbg: {
      __wbg_log_12edb8942696c207: (p,n)=>{
        console.log(new TextDecoder().decode(new Uint8Array(wasm.memory.buffer).subarray(p,p+n)));
      }
    }
  };
  const {instance: {exports: wasm}}=await WebAssembly.instantiateStreaming(await fetch('./avif.wasm',{cache: 'force-cache'}),imports);
  const malloc=wasm.__wbindgen_malloc;
  const free=wasm.__wbindgen_free;
  const pointer=wasm.__wbindgen_add_to_stack_pointer;
  return (it,width,height,quality=50,speed=6)=>{
    const n1=it.length;
    const p1=malloc(n1,1);
    const r=pointer(-16);
    try{
      new Uint8Array(wasm.memory.buffer).set(data,p1);
      wasm.avif_from_imagedata(r,p1,n1,width,height,quality,speed);
      const arr=new Int32Array(wasm.memory.buffer);
      const p2=arr[r/4];
      const n2=arr[r/4+1];
      const res=new Uint8Array(wasm.memory.buffer).subarray(p2,p2+n2).slice();
      free(p2,n2);
      return res;
    }finally{
      pointer(16)
    }
  };
})();
