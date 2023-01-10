#![allow(dead_code, non_snake_case, non_upper_case_globals)]

mod components;

use components::App;

fn main()
{
	wasm_logger::init(wasm_logger::Config::default());
	dioxus::web::launch(App);
}
