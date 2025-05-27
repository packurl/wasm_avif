const url=new URL('avif.wasm',import.meta.url);
let wasm;
const imports={
  wbg: {
    __wbg_log_c7ab787585a0044f: (p,n)=>{
      console.log(new TextDecoder().decode(new Uint8Array(wasm.memory.buffer).subarray(p,p+n)));
    },
    __wbindgen_init_externref_table:function(){
      const table=wasm.__wbindgen_export_0;
      const offset=table.grow(4);
      table.set(0);
      table.set(offset);
      table.set(offset+1,null);
      table.set(offset+2,true);
      table.set(offset+3,false);
    }
  }
};
const {instance}=await WebAssembly.instantiateStreaming(await fetch(url,{cache: 'force-cache'}),imports);
wasm=instance.exports;
const malloc=wasm.__wbindgen_malloc;
const free=wasm.__wbindgen_free;
/**
 * Encodes the supplied ImageData rgba array.
 * @param {Uint8Array} bytes
 * @param {number} width
 * @param {number} height
 * @param {number} quality (1 to 100)
 * @param {number} speed (1 to 10)
 * @return {Uint8Array}
 */
const avif=(bytes,width,height,quality=50,speed=6)=>{
  const n1=bytes.length;
  const p1=malloc(n1,1);
  new Uint8Array(wasm.memory.buffer).set(bytes,p1);
  const [p2,n2]=wasm.avif_from_imagedata(p1,n1,width,height,quality,speed);
  const res=new Uint8Array(wasm.memory.buffer).subarray(p2,p2+n2).slice();
  free(p2,n2);
  return res;
};
export {avif};
export default avif;
