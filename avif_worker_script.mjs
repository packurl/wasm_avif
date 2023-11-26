import {avif} from "./avif.mjs";
onmessage=async({data:{bytes,width,height,quality,speed}})=>postMessage(avif(bytes,width,height,quality,speed));
postMessage('ready');