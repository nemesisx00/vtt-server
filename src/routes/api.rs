#![allow(dead_code, non_snake_case, non_upper_case_globals)]

use crate::{
	api::structs::*,
	database::{
		findUserById,
		getDatabaseConnection,
	},
	entities::*,
};
use actix_web::{
	get,
	post,
	HttpResponse,
	Responder,
};
use sea_orm::{
	EntityTrait,
	Set, ActiveModelTrait,
};

#[post("/echo")]
pub async fn echo(reqBody: String) -> impl Responder
{
	return HttpResponse::Ok().body(reqBody);
}

#[get("/generateUser")]
pub async fn generateUser() -> impl Responder
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
pub async fn home() -> impl Responder
{
	return HttpResponse::Ok().body("Hello VTT World!");
}

#[post("/login")]
pub async fn login(json: String) -> impl Responder
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
