#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::{
	api::structs::*,
	database::{
		findUserById, getDatabaseConnection,
	},
};
use actix_web::{
	get,
	post,
	HttpResponse,
	Responder,
};

#[post("/echo")]
pub async fn echo(reqBody: String) -> impl Responder
{
	return HttpResponse::Ok().body(reqBody);
}

#[get("/")]
pub async fn home() -> impl Responder
{
	return HttpResponse::Ok().body("Hello VTT World!");
}

#[post("/login")]
pub async fn login(json: String) -> impl Responder
{
	let db = getDatabaseConnection().await.unwrap();
	let loginData: LoginData = serde_json::from_str(&json).unwrap();
	
	let mut resp = LoginResponseData
	{
		Message: "No User found!".to_string(),
		..Default::default()
	};
	
	if let Some(userId) = loginData.UserId
	{
		if let Ok(Some(u)) = findUserById(&db, userId).await
		{
			resp.UserId = Some(u.id);
			resp.Message = format!("Hello {}!", u.label);
		}
	}
	
	let respJson = serde_json::to_string(&resp).unwrap();
	return HttpResponse::Ok().body(respJson);
}
