#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use std::error::Error;
use reqwest::Url;
use log::error;
use wasm_bindgen::prelude::*;
#[allow(unused_imports)]
use web_sys::Window;

#[wasm_bindgen]
pub fn location() -> String
{
	let mut url = "".to_owned();
	if let Some(window) = web_sys::window()
	{
		let location = window.location().to_string();
		url = location.into();
	}
	else
	{
		error!("Failed to access window.location!");
	}
	
	return url;
}

pub fn endpoint(path: &str) -> Result<String, Box<dyn Error>>
{
	let endpoint = Url::parse(location().as_ref())?
		.join(path)?
		.to_string();
	return Ok(endpoint);
}
