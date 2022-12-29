#![allow(dead_code, non_snake_case, non_upper_case_globals)]

use crate::migrations::createAllTables;
use sea_orm::{
	Database,
	DatabaseConnection,
	DbErr,
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
