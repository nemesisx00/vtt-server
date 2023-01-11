#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::entities::*;
use sea_orm::{
	ConnectionTrait,
	DatabaseConnection,
	DbErr,
	EntityTrait,
	Schema,
};

async fn createTable<E>(db: &DatabaseConnection, entity: E)
	where E: EntityTrait
{
	let backend = db.get_database_backend();
	let schema = Schema::new(backend);
	let stmt = backend.build(schema.create_table_from_entity(entity).if_not_exists());
	
	match db.execute(stmt).await
	{
		Ok(_) => {},//println!("Migrated {}", entity.table_name()),
		Err(e) => println!("Error: {}", e),
	};
}

pub async fn createAllTables(db: &DatabaseConnection) -> Result<(), DbErr>
{
	createTable(db, user::Entity).await;
	createTable(db, token::Entity).await;
	
	return Ok(());
}

#[cfg(test)]
mod tests
{
	#[allow(unused_imports)]
	use super::*;
	use crate::database::createTestDatabase;
	use sea_orm::{
		JsonValue,
		Statement,
		DbBackend,
		FromQueryResult,
	};
	
	#[actix_web::test]
	async fn test_createAllTables()
	{
		let db = createTestDatabase().await.unwrap();
		let rows: Vec<JsonValue> = JsonValue::find_by_statement(Statement::from_sql_and_values(
			DbBackend::Sqlite,
			r#"SELECT name FROM sqlite_schema WHERE type = 'table' AND name NOT LIKE 'sqlite_%'"#,
			vec![],
		))
		.all(&db)
		.await
		.unwrap();
		
		assert!(rows.len() > 0);
	}
}
