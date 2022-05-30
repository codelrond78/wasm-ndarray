import * as wasm from "wasm-ndarray";

//wasm.greet();
let ini = + new Date()
console.log(wasm.multiply({field1: {}, field2: [[1.0, 2.0], [3.1, 3.3]] }))
let now = + new Date()
//--
console.log("time it", now - ini)
