#![allow(dead_code, non_snake_case, non_upper_case_globals)]

use crate::entities::*;
use sea_orm::{
	ConnectionTrait,
	DatabaseConnection,
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
		Ok(_) => println!("Migrated {}", entity.table_name()),
		Err(e) => println!("Error: {}", e),
	};
}

pub async fn createTables(db: &DatabaseConnection)
{
	createTable(db, User).await;
}
