#![allow(dead_code, non_snake_case, non_upper_case_globals)]

use crate::{
	config::{
		loadConfig,
		ConfigPath,
	},
	entities::*,
	migrations::createAllTables,
};
use std::error::Error;
use sea_orm::{
	Database,
	DatabaseConnection,
	EntityTrait, QueryOrder,
};

pub async fn getDatabaseConnection() -> Result<DatabaseConnection, Box<dyn Error>>
{
	let config = loadConfig(ConfigPath)?;
	let db = Database::connect(config.database.connectionString).await?;
	createAllTables(&db).await?;
	return Ok(db);
}

pub async fn findUserById(id: i64) -> Option<User>
{
	let mut user = None;
	if let Ok(db) = getDatabaseConnection().await
	{
		if let Ok(u) = user::Entity::find_by_id(id).one(&db).await
		{
			user = u;
		}
	}
	
	return user;
}

pub async fn allUsers() -> Vec<User>
{
	let mut users: Vec<User> = vec![];
	if let Ok(db) = getDatabaseConnection().await
	{
		if let Ok(found) = user::Entity::find().order_by_asc(user::Column::Label).all(&db).await
		{
			users = found.to_owned();
		}
	}
	
	return users;
}
