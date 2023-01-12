#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::{
	config::{
		loadConfig,
		ConfigPath,
	},
};
use std::error::Error;
use sea_orm::{
	Database,
	DatabaseConnection,
};

pub async fn getDatabaseConnection() -> Result<DatabaseConnection, Box<dyn Error>>
{
	let config = loadConfig(ConfigPath)?;
	let db = Database::connect(config.database.connectionString).await?;
	return Ok(db);
}

#[cfg(test)]
pub mod tests
{
	use crate::{
		migrations::createAllTables,
	};
	use std::error::Error;
	use sea_orm::{
		Database,
		DatabaseConnection,
	};
	
	const TestConnString: &str = "sqlite::memory:";

	pub async fn createTestDatabase() -> Result<DatabaseConnection, Box<dyn Error>>
	{
		let db = Database::connect(TestConnString).await?;
		createAllTables(&db).await?;
		return Ok(db);
	}
}
