#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

mod components;
mod structs;
mod util;

use crate::components::App;

fn main()
{
	wasm_logger::init(wasm_logger::Config::default());
	dioxus::web::launch(App);
}
