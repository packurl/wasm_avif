import {avif} from "./avif.mjs";
onmessage=async({data:{bytes,width,height,quality,speed}})=>{
  const res=avif(bytes,width,height,quality,speed);
  postMessage(res,[res.buffer]);
}
postMessage('ready');