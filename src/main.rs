#![allow(dead_code, non_snake_case, non_upper_case_globals)]

mod entities;
mod migrations;

use crate::migrations::createTables;
use actix_web::{
	get,
	post,
	App,
	HttpResponse,
	HttpServer,
	Responder,
};
use futures::executor::block_on;
use sea_orm::{
	Database,
	DbErr,
};
use sea_orm_migration::prelude::*;

/*
MySQL				mysql://root:root@localhost:3306
PostgreSQL			postgres://root:root@localhost:5432
SQLite (in file)	sqlite:./sqlite.db?mode=rwc
SQLite (in memory)	sqlite::memory:
*/
const DatabaseConnectionString: &str = "sqlite:./sqlite.db?mode=rwc";
const DatabaseName: &str = "test";

async fn runMigrations() -> Result<(), DbErr>
{
	let db = Database::connect(DatabaseConnectionString).await?;
	createTables(&db).await;
	return Ok(());
}

#[get("/")]
async fn hello() -> impl Responder
{
	HttpResponse::Ok().body("Hello VTT World!")
}

#[post("/echo")]
async fn echo(reqBody: String) -> impl Responder
{
	HttpResponse::Ok().body(reqBody)
}

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
	if let Err(err) = block_on(runMigrations())
	{
		panic!("{}", err);
	}
	
	return HttpServer::new(||
	{
		App::new()
			.service(hello)
			.service(echo)
	})
		.bind(("127.0.0.1", 8080))?
		.run()
		.await;
}
