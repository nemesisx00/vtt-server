#![allow(dead_code, non_snake_case, non_upper_case_globals)]

use crate::{
	config::{
		loadConfig,
		ConfigPath,
	},
	entities::*,
	migrations::createAllTables,
};
use sea_orm::{
	Database,
	DatabaseConnection,
	DbErr,
	EntityTrait,
};

pub async fn getDatabaseConnection() -> Result<DatabaseConnection, DbErr>
{
	let config = loadConfig(ConfigPath);
	let db = Database::connect(config.database.connectionString)
		.await
		.expect("Failed to connect to database");
	
	createAllTables(&db)
		.await
		.expect("Failed to run database migrations.");
	
	return Ok(db);
}

pub async fn findUserById(id: i64) -> Option<user::Model>
{
	let mut user = None;
	if let Ok(db) = getDatabaseConnection().await
	{
		if let Ok(u) = User::find_by_id(id).one(&db).await
		{
			user = u;
		}
	}
	
	return user;
}
