use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use log::debug;

/**
    HashMap is not exposable: return HashMap: "the trait IntoWasmAbi is not implemented for HashMap<String, Vec<f64>>"
    Workaround: Use handle or do it like this

    NOTE : This does not handle newlines or spaces or any other exceptions,
    but as the intention of post is to show connecting Rust and JS,
    we consider only ideal conditions and not errors for the parsing functions

    returns: JsValue: lives in JS world
**/
#[wasm_bindgen]
pub fn parse(input: &str) -> JsValue {
    debug!("[WASM] Parsing {:?}", input);

    let mut ret: HashMap<String, Vec<f32>> = HashMap::new();

    let (keys, values) = input.split_once(';').unwrap();
    let keys: Vec<_> = keys.split(',').collect();

    let mut temp: Vec<Vec<f32>> = Vec::with_capacity(keys.len());
    for _ in 0..keys.len() {
        temp.push(Vec::new());
    }
    for row in values.split(';') {
        for (i, v) in row.split(',').enumerate() {
            temp[i].push(v.parse().unwrap());
        }
    }
    for (k, v) in keys.into_iter().zip(temp.into_iter()) {
        ret.insert(k.to_owned(), v);
    }

    debug!("[WASM] Parsed to {:?}", ret);
    JsValue::from_serde(&ret).unwrap()
}