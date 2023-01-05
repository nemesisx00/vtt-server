#![allow(dead_code, non_snake_case, non_upper_case_globals)]

mod components;

use components::App;

fn main()
{
	dioxus::web::launch(App);
}
