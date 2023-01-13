#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

mod api;
mod config;
mod database;
mod entities;
mod error;
mod migrations;
mod routes;

use crate::{
	config::loadConfig,
	database::getDatabaseConnection,
	migrations::createAllTables,
};
use futures::future;
use actix_web::{
	web::{
		Data,
	},
	App,
	HttpServer,
};

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
	let config = loadConfig().unwrap();
	
	let db = getDatabaseConnection().await.expect("Failed to connect to the database!");
	createAllTables(&db).await.expect("Failed to run initial database migrations!");
	let db2 = db.to_owned();
	
	let server = HttpServer::new(move ||
	{
		App::new()
			.app_data(Data::new(db.to_owned()))
			.service(routes::api::root)
			.service(routes::api::login)
	})
		.bind((config.network.ip, config.network.port))?
		.run();
	
	let admin = HttpServer::new(move ||
		{
			App::new()
				.app_data(Data::new(db2.to_owned()))
				.service(routes::admin::core::home)
				.service(routes::admin::core::web)
				.service(routes::admin::user::userDelete)
				.service(routes::admin::user::userList)
				.service(routes::admin::user::userNew)
				.service(routes::admin::user::userUpdate)
		})
			.bind((config.admin.ip, config.admin.port))?
			.run();
	
	future::try_join(server, admin).await?;
	return Ok(());
}
