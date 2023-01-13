#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use wasm_bindgen::prelude::*;
#[allow(unused_imports)]
use web_sys::Window;

#[wasm_bindgen]
pub fn location() -> String
{
	let window = web_sys::window().unwrap();
	let location = window.location().to_string();
	return location.into();
}
