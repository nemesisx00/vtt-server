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
	config::{
		loadConfig,
		ConfigPath,
	},
	database::getDatabaseConnection,
	migrations::createAllTables,
};
use actix_web::{
	web::{
		self,
		Data,
	},
	App,
	HttpServer,
};

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
	let config = loadConfig(ConfigPath).unwrap();
	
	let db = getDatabaseConnection().await.expect("Failed to connect to the database!");
	createAllTables(&db).await.expect("Failed to run initial database migrations!");
	
	return HttpServer::new(move ||
	{
		App::new()
			.app_data(Data::new(db.to_owned()))
			.service(routes::api::echo)
			.service(routes::api::home)
			.service(routes::api::login)
			.service(
				web::scope("/admin")
					.service(routes::admin::core::home)
					.service(routes::admin::core::web)
					.service(routes::admin::user::userList)
					.service(routes::admin::user::userNew)
			)
	})
		.bind((config.network.ip, config.network.port))?
		.run()
		.await;
}
