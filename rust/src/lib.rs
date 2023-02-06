pub mod parser; // make public to JS

/**
WASM bridge:
 provides the macros and other items needed to compile and bind Rust code to Wasm
 The way all of this works is that wasm_bindgen implements the derive function, which converts Rust values to JavaScript and vice versa.
**/
use wasm_bindgen::prelude::*;
use log::Level;
use log::info;
use log::debug;

#[wasm_bindgen]
pub fn main() {
    console_log::init_with_level(Level::Debug);
    debug!("[WASM] lib initialized!");
}

#[wasm_bindgen]
pub struct Car {
    pub number: usize, // immutable
    pub color:usize, // color in hex code

    // WASM: cannot be passed to JS easily since JS allows mutability (and Rust not always) + GC complicated if both sides involved (Error: "trait is not satisfied") -> not directly pub.
    owner: OwnerID,
    boxed_value: Option<Box<usize>>
}
#[derive(Clone)] // macro to enable clone() if alle types are primitive or cloneable
pub struct OwnerID {
    id: usize,
}

#[wasm_bindgen]
// structs don't have constructors / method
impl Car {
    pub fn new() -> Self {
        Car {
            number: 0,
            color: 0,
            owner: OwnerID { id: 0 },
            boxed_value: Some(Box::new(0)) // optionals construction
        }
    }

    pub fn duplicate(&self) -> Self {
        Self {
            number: self.number + 1,
            color: self.color,
            owner: self.owner.clone(),
            boxed_value: None // optionals must be given
        }
    }

    pub fn get_owner_id(&self) -> usize {
        self.owner.id
    }

    // takes self mutably
    pub fn change_color(mut self, new_color:usize) {
        self.color = new_color;
    }
}

#[wasm_bindgen]
pub fn color(mut a: Car, color:usize) -> Car {
    a.color = color;
    return a;
}

#[wasm_bindgen]
pub fn add(a: usize, b: usize) -> usize {
    debug!("[WASM] Adding {:?} + {:?}", a, b);
    return a + b;
}

// ################### WASM-Javascript code bridge #################
#[wasm_bindgen]
extern "C" { // bridge to JS via foreign function interfaces (FFI)
    fn alert(s: &str); // exposes JS alert to Rust
    #[wasm_bindgen(js_name = alert)] // "overloaded" call to JS alert
    fn alert_usize(a: usize);
}
#[wasm_bindgen]
pub fn js_alert() {
    alert("This is a JS alert called from rust!");
}
#[wasm_bindgen]
pub fn js_alert_number() {
    alert_usize(1000000);
}