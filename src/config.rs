#![allow(dead_code, non_snake_case, non_upper_case_globals)]

use std::{
	fs::File,
	io::BufReader,
};
use serde::{
	Deserialize,
	Serialize,
};

pub const ConfigPath: &str = "./config.json";

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
	pub database: ConfigDatabase,
	pub network: ConfigNetwork,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ConfigDatabase
{
	/*
	MySQL				mysql://root:root@localhost:3306
	PostgreSQL			postgres://root:root@localhost:5432
	SQLite (in file)	sqlite:./sqlite.db?mode=rwc
	SQLite (in memory)	sqlite::memory:
	*/
	pub connectionString: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ConfigNetwork
{
	pub ip: String,
	pub port: u16,
}
