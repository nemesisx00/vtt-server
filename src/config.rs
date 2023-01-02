#![allow(dead_code, non_snake_case, non_upper_case_globals)]

use std::{
	fs::File,
	io::BufReader,
};
use serde::{
	Deserialize,
	Serialize,
};

pub fn loadConfig(path: &str) -> Config
{
	let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
	let config: Config = serde_json::from_reader(reader).unwrap();
	return config;
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config
{
	pub network: ConfigNetwork,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ConfigNetwork
{
	pub ip: String,
	pub port: u16,
}
