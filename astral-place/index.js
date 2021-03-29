import * as wasm from "hello-wasm-pack";
import * as astral from "@xavulu/astral_wasm";
wasm.greet();
let data = new Uint8Array([1,2,3,4,5,6,7,8,9,10]); 
let pass = "hello";
let enc = astral.encrypt_data(data, pass, false); 
let dec = astral.decrypt_data(enc, pass); 
console.log(enc); 
console.log(dec);