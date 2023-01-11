#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::{
	config::{
		loadConfig,
		ConfigPath,
	},
	entities::*,
	error::MessageError,
	migrations::createAllTables,
};
use std::error::Error;
use sea_orm::{
	ActiveModelTrait,
	ColumnTrait,
	Database,
	DatabaseConnection,
	EntityTrait,
	QueryOrder,
	QueryFilter,
	Set,
};

const TestConnString: &str = "sqlite::memory:";

pub async fn createTestDatabase() -> Result<DatabaseConnection, Box<dyn Error>>
{
	let db = Database::connect(TestConnString).await?;
	createAllTables(&db).await?;
	return Ok(db);
}

pub async fn getDatabaseConnection() -> Result<DatabaseConnection, Box<dyn Error>>
{
	let config = loadConfig(ConfigPath)?;
	let db = Database::connect(config.database.connectionString).await?;
	return Ok(db);
}

pub async fn createUser(db: &DatabaseConnection, username: String, label: String) -> Result<User, Box<dyn Error>>
{
	if let Ok(Some(_)) = user::Entity::find()
		.filter(user::Column::Username.contains(&username))
		.one(db)
		.await
	{
		return Err(Box::new(MessageError { message: format!("User with username '{}' already exists!", username) }));
	}
	
	let newUser = UserActive
	{
		username: Set(username.to_owned()),
		label: Set(label.to_owned()),
		..Default::default()
	};
	
	let user = newUser.insert(db).await?;
	return Ok(user);
}

pub async fn findUserById(db: &DatabaseConnection, id: i64) -> Result<Option<User>, Box<dyn Error>>
{
	let result = user::Entity::find_by_id(id)
		.one(db)
		.await?;
	
	return Ok(result);
}

pub async fn findAllUsers(db: &DatabaseConnection) -> Result<Vec<User>, Box<dyn Error>>
{
	let result = user::Entity::find()
		.order_by_asc(user::Column::Label)
		.all(db)
		.await?;
	
	return Ok(result);
}

#[cfg(test)]
mod tests
{
	use super::*;
	
	async fn createTestUser(db: &DatabaseConnection) -> User
	{
		let newUser = UserActive
		{
			username: Set("test".to_owned()),
			label: Set("test".to_owned()),
			..Default::default()
		};
		
		let user = newUser.insert(db).await.unwrap();
		return user;
	}
	
	#[actix_web::test]
	async fn test_createUser()
	{
		let username = "username".to_owned();
		let label = "label".to_owned();
		
		let db = createTestDatabase().await.unwrap();
		let user = createUser(&db, username.to_owned(), label.to_owned()).await.unwrap();
		assert_eq!(user.id, 1);
		assert_eq!(user.username, username);
		assert_eq!(user.label, label);
		
		let users = findAllUsers(&db).await.unwrap();
		assert_eq!(users.len(), 1);
		assert_eq!(users[0], user);
	}
	
	#[actix_web::test]
	async fn test_findAllUsers()
	{
		let db = createTestDatabase().await.unwrap();
		createTestUser(&db).await;
		let users = findAllUsers(&db).await.unwrap();
		assert_eq!(users.len(), 1);
	}
	
	#[actix_web::test]
	async fn test_findUserById()
	{
		let db = createTestDatabase().await.unwrap();
		
		let mut result = findUserById(&db, 1).await.unwrap();
		assert_eq!(result, None);
		
		let expected = createTestUser(&db).await;
		result = findUserById(&db, 1).await.unwrap();
		assert_eq!(result.unwrap(), expected);
	}
	
	#[actix_web::test]
	async fn test_entityUpdate()
	{
		let expected = "test2".to_owned();
		
		let db = createTestDatabase().await.unwrap();
		
		let user = createTestUser(&db).await;
		let generated = user.username.to_owned();
		
		let mut active: UserActive = user.into();
		active.username = Set(expected.to_owned());
		
		let before = findUserById(&db, 1).await.unwrap().unwrap();
		assert_ne!(before.username, expected);
		assert_eq!(before.username, generated);
		
		let after = active.update(&db).await.unwrap();
		assert_eq!(after.username, expected);
		assert_ne!(after, before);
		
		let found = findUserById(&db, 1).await.unwrap().unwrap();
		assert_ne!(found, before);
		assert_eq!(found, after);
	}
}
