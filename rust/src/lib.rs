pub(crate) mod core;

use std::collections::HashMap;
use serde::Serialize;
use wasm_bindgen::prelude::*;
use web_sys::console;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// This is like the `main` function, except for JavaScript.
#[wasm_bindgen]
pub fn greet() -> Result<JsValue, JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
        console_error_panic_hook::set_once();

    // Your code goes here!
    let result = JsValue::from_str("Hello from Rust!");
    console::log_1(&result);

    Ok(result)
}

#[wasm_bindgen]
pub struct Person {
    pub age: u32,
}

#[wasm_bindgen]
pub fn get_object() -> Result<JsValue,JsValue> {
    Ok(Person { age: 10 }.into())
}

#[wasm_bindgen]
pub fn throw_exception() -> Result<(),JsValue> {
    Err("Error message".into())
}

// #[wasm_bindgen]
#[derive(Serialize)]
pub struct Response {
    pub matchups: HashMap<String, Matchup>
}

// #[wasm_bindgen]
#[derive(Serialize)]
pub struct Matchup {
    opponent: String,
    outcome: String,
    strategy: String,
}
#[wasm_bindgen]
pub fn get_matchups() -> JsValue {
    // let mut matchups = HashMap::new();
    // matchups.insert("Mengsk".to_string(), Matchup {
    //     opponent: "Dehaka".to_string(),
    //     outcome: "Even".to_string(),
    //     strategy: "No royal guard lol".to_string()
    // });
    // matchups.insert("Mengsk".to_string(), Matchup {
    //     opponent: "Tychus".to_string(),
    //     outcome: "Bad".to_string(),
    //     strategy: "you lose".to_string()
    // });
    // matchups.insert("Alarak".to_string(), Matchup {
    //     opponent: "Tychus".to_string(),
    //     outcome: "Bad".to_string(),
    //     strategy: "you lose also".to_string()
    // });
    //
    // serde_wasm_bindgen::to_value(&    Response {
    //     matchups,
    // }).unwrap()
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    match core::get_matchups() {
        Ok(matchups) => match serde_wasm_bindgen::to_value(&matchups) {
            Ok(result) => result,
            Err(e) => format!("Failed to convert to js: {:?}", e).into(),
        }
        Err(e) => format!("Failed to get matchups: {:?}", e).into()
    }
    // let matchups = core::get_matchups().expect("failed to get matchups");
    // serde_wasm_bindgen::to_value(&matchups).expect("failed to convert to js")
}
