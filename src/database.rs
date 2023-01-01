#![allow(dead_code, non_snake_case, non_upper_case_globals)]

use crate::{
	entities::*,
	migrations::createAllTables,
};
use sea_orm::{
	Database,
	DatabaseConnection,
	DbErr,
	EntityTrait,
};

/*
MySQL				mysql://root:root@localhost:3306
PostgreSQL			postgres://root:root@localhost:5432
SQLite (in file)	sqlite:./sqlite.db?mode=rwc
SQLite (in memory)	sqlite::memory:
*/
const DatabaseConnectionString: &str = "sqlite:./sqlite.db?mode=rwc";

pub async fn getDatabaseConnection() -> Result<DatabaseConnection, DbErr>
{
	let db = Database::connect(DatabaseConnectionString)
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
