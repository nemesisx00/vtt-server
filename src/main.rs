#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

mod api;
mod config;
mod database;
mod entities;
mod error;
mod middleware;
mod migrations;
mod routes;

use crate::{
	config::loadConfig,
	database::getDatabaseConnection,
	middleware::auth::AuthenticationService,
	migrations::createAllTables,
};
use futures::future;
use actix_web::{
	web::Data,
	App,
	HttpServer,
};
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
	env_logger::init_from_env(Env::default().default_filter_or("info"));
	
	let config = loadConfig().unwrap();
	
	let db = getDatabaseConnection().await.expect("Failed to connect to the database!");
	createAllTables(&db).await.expect("Failed to run initial database migrations!");
	let db2 = db.to_owned();
	
	let api = HttpServer::new(move ||
	{
		App::new()
			.app_data(Data::new(db.to_owned()))
			.wrap(AuthenticationService)
			.service(routes::api::root::root)
			.service(routes::api::root::login)
			.service(routes::api::token::getToken)
	})
		.bind((config.network.ip, config.network.port))?
		.run();
	
	let admin = HttpServer::new(move ||
	{
		App::new()
			.app_data(Data::new(db2.to_owned()))
			.wrap(AuthenticationService)
			.service(routes::admin::core::home)
			.service(routes::admin::core::web)
			.service(routes::admin::user::userDelete)
			.service(routes::admin::user::userList)
			.service(routes::admin::user::userNew)
			.service(routes::admin::user::userUpdate)
	})
		.bind((config.admin.ip, config.admin.port))?
		.run();
	
	future::try_join(api, admin).await?;
	return Ok(());
}
