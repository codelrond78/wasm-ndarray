mod utils;

use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
#[macro_use]
extern crate ndarray;

use ndarray::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Example {
    pub field1: HashMap<u32, String>,
    pub field2: Vec<Vec<f32>>
    //pub field3: [f32; 4],
    //pub field3: Array2::<f64>
}

#[derive(Serialize, Deserialize)]
pub struct ResponseExample {
    pub field1: HashMap<u32, String>,
    pub field2: Array2::<f64>
    //pub field3: [f32; 4],
    //pub field3: Array2::<f64>
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-ndarray!");
}

#[wasm_bindgen]
pub fn multiply(val: &JsValue) -> JsValue{
    let arg: Example = val.into_serde().unwrap();

    let mut arr = Array2::<f64>::zeros((2,2));
    arr[ [0, 0] ] = -1.1;

    let mut field1 = HashMap::new();
    field1.insert(0, String::from("ok! ;)"));
    let example = ResponseExample {
        field1,
        field2: arr//arg.field2
    };

    JsValue::from_serde(&example).unwrap()
}