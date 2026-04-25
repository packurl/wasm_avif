const url=new URL('avif.wasm',import.meta.url);
let wasm;
const imports={
  js: {
    println:(ptr,len)=>console.log(new TextDecoder().decode(new Uint8Array(wasm.memory.buffer,ptr,len)))
  }
};
const {instance}=await WebAssembly.instantiateStreaming(await fetch(url,{cache: 'force-cache'}),imports);
wasm=instance.exports;
const malloc=wasm.malloc;
const free=wasm.free;
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
  const r=wasm.avif_from_imagedata(p1,n1,width,height,quality,speed);
  const ptr_and_len=new DataView(wasm.memory.buffer,r,8);
  const p2=ptr_and_len.getUint32(0,true);
  const n2=ptr_and_len.getUint32(4,true);
  free(ptr_and_len,8);
  const res=new Uint8Array(wasm.memory.buffer).subarray(p2,p2+n2).slice();
  free(p2,n2);
  return res;
};
export {avif};
export default avif;
