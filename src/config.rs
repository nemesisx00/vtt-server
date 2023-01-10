#![allow(dead_code, non_snake_case, non_upper_case_globals)]

use std::{
	error::Error,
	fs,
};
use serde::Deserialize;
use toml;

pub const ConfigPath: &str = "./config.toml";

pub fn loadConfig(path: &str) -> Result<Config, Box<dyn Error>>
{
	let config = toml::from_str::<Config>(fs::read_to_string(path)?.as_str())?;
	return Ok(config);
}

#[derive(Debug, Default, Deserialize)]
pub struct Config
{
	pub database: ConfigDatabase,
	pub network: ConfigNetwork,
}

#[derive(Debug, Default, Deserialize)]
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

#[derive(Debug, Default, Deserialize)]
pub struct ConfigNetwork
{
	pub ip: String,
	pub port: u16,
}
