#![allow(dead_code, non_snake_case, non_upper_case_globals)]

mod api;
mod config;
mod database;
mod entities;
mod migrations;

use crate::{
	api::structs::*,
	config::loadConfig,
	database::{
		findUserById,
		getDatabaseConnection,
	},
	entities::*,
};
use actix_web::{
	get,
	post,
	App,
	HttpResponse,
	HttpServer,
	Responder,
};
use sea_orm::{
	EntityTrait,
	Set, ActiveModelTrait,
};

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
	let config = loadConfig("./config.json");
	
	return HttpServer::new(||
	{
		App::new()
			.service(echo)
			.service(generateUser)
			.service(hello)
			.service(login)
	})
		.bind((config.network.ip, config.network.port))?
		.run()
		.await;
}

#[post("/echo")]
async fn echo(reqBody: String) -> impl Responder
{
	return HttpResponse::Ok().body(reqBody);
}

#[get("/generateUser")]
async fn generateUser() -> impl Responder
{
	let mut resp = "Failed to create new User.".to_string();
	if let Ok(db) = getDatabaseConnection().await
	{
		if let Ok(Some(_)) = User::find_by_id(1).one(&db).await
		{
			resp = "User already exists!".to_string();
		}
		else
		{
			let u = user::ActiveModel
			{
				label: Set("John Doe".to_owned()),
				name: Set(Some("Jake Smith".to_owned())),
				..Default::default()
			};
			
			match u.insert(&db).await
			{
				Ok(newUser) => { resp = format!("Created new User with id: {}", newUser.id); },
				Err(e) => println!("Failed to insert new User: {}", e),
			}
		}
	}
	
	return HttpResponse::Ok().body(resp);
}

#[get("/")]
async fn hello() -> impl Responder
{
	return HttpResponse::Ok().body("Hello VTT World!");
}

#[post("/login")]
async fn login(json: String) -> impl Responder
{
	let loginData: LoginData = serde_json::from_str(&json).unwrap();
	
	let mut resp = LoginResponseData
	{
		Message: "No User found!".to_string(),
		..Default::default()
	};
	
	if let Some(userId) = loginData.UserId
	{
		if let Some(u) = findUserById(userId).await
		{
			resp.UserId = Some(u.id);
			resp.Message = format!("Hello {}!", u.label);
		}
	}
	
	let respJson = serde_json::to_string(&resp).unwrap();
	return HttpResponse::Ok().body(respJson);
}
