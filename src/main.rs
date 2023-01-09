#![allow(dead_code, non_snake_case, non_upper_case_globals)]

mod api;
mod config;
mod database;
mod entities;
mod migrations;
mod routes;

use crate::{
	config::{
		loadConfig,
		ConfigPath,
	},
};
use actix_web::{
	web,
	App,
	HttpServer,
};

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
	let config = loadConfig(ConfigPath).unwrap();
	
	return HttpServer::new(||
	{
		App::new()
			.service(routes::api::echo)
			.service(routes::api::generateUser)
			.service(routes::api::home)
			.service(routes::api::login)
			.service(
				web::scope("/admin")
					.service(routes::admin::home)
					.service(routes::admin::getFile)
					.service(routes::admin::getInterpreter)
					.service(routes::admin::userList)
			)
	})
		.bind((config.network.ip, config.network.port))?
		.run()
		.await;
}
