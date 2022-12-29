#![allow(dead_code, non_snake_case, non_upper_case_globals)]

mod database;
mod entities;
mod migrations;

use crate::{
	entities::*,
	database::getDatabaseConnection,
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

#[get("/")]
async fn hello() -> impl Responder
{
	let mut user = None;
	if let Ok(db) = getDatabaseConnection().await
	{
		if let Ok(u) = User::find_by_id(1).one(&db).await
		{
			user = u;
		}
	}
	
	let resp = match user
	{
		Some(u) => format!("Hello {}!", u.label),
		None => "Hello VTT World! No User found!".to_string(),
	};
	
	return HttpResponse::Ok().body(resp);
}

#[post("/echo")]
async fn echo(reqBody: String) -> impl Responder
{
	HttpResponse::Ok().body(reqBody)
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
				Err(e) => println!("{}", e),
			}
		}
	}
	
	return HttpResponse::Ok().body(resp);
}

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
	return HttpServer::new(||
	{
		App::new()
			.service(hello)
			.service(echo)
			.service(generateUser)
	})
		.bind(("127.0.0.1", 8080))?
		.run()
		.await;
}
