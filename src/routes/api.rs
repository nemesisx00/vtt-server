#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::{
	api::structs::*,
	database::{
		findUserById,
	},
};
use actix_web::{
	get,
	post,
	web::{
		Data,
		Form,
	},
	HttpResponse,
	Responder,
};
use sea_orm::DatabaseConnection;

#[get("/")]
pub async fn root() -> impl Responder
{
	return HttpResponse::Ok().body("Hello VTT World!");
}

#[post("/login")]
pub async fn login(data: Form<LoginData>, db: Data<DatabaseConnection>) -> impl Responder
{
	let mut resp = LoginResponseData
	{
		message: "No User found!".to_owned(),
		..Default::default()
	};
	
	if let Some(userId) = data.userId
	{
		if let Ok(Some(u)) = findUserById(&db, userId).await
		{
			resp.userId = Some(u.id);
			resp.message = format!("Hello {}!", u.label);
		}
	}
	
	let respJson = serde_json::to_string(&resp).unwrap();
	return HttpResponse::Ok().body(respJson);
}

#[cfg(test)]
mod tests
{
	use crate::{
		api::structs::{
			LoginData,
			LoginResponseData,
		},
		database::{
			createTestDatabase,
			createUser,
		},
		routes,
	};
	use actix_web::{
		test::{
			self,
			TestRequest,
		},
		web::{
			Data,
		},
		App,
	};
	
	#[actix_web::test]
	async fn test_login_empty()
	{
		let uri = "/login";
		
		let db = createTestDatabase().await.unwrap();
		let app = test::init_service(
			App::new()
				.app_data(Data::new(db.to_owned()))
				.service(routes::api::login)
		).await;
		
		// No userId provided!
		let data = LoginData { userId: None };
		let req = TestRequest::post()
			.uri(uri)
			.set_form(data)
			.to_request();
		
		let resp: LoginResponseData = test::call_and_read_body_json(&app, req).await;
		assert_eq!(resp.userId, None);
		assert_eq!(resp.message, "No User found!".to_owned());
	}
	
	#[actix_web::test]
	async fn test_login_invalid()
	{
		let uri = "/login";
		let expectedId = 1;
		
		let db = createTestDatabase().await.unwrap();
		let app = test::init_service(
			App::new()
				.app_data(Data::new(db.to_owned()))
				.service(routes::api::login)
		).await;
		
		let data = LoginData { userId: Some(expectedId) };
		let req = TestRequest::post()
			.uri(uri)
			.set_form(data)
			.to_request();
		
		let resp: LoginResponseData = test::call_and_read_body_json(&app, req).await;
		assert_eq!(resp.userId, None);
		assert_eq!(resp.message, "No User found!".to_owned());
	}
	
	#[actix_web::test]
	async fn test_login_valid()
	{
		let uri = "/login";
		let username = "username".to_owned();
		let label = "label".to_owned();
		
		let db = createTestDatabase().await.unwrap();
		let app = test::init_service(
			App::new()
				.app_data(Data::new(db.to_owned()))
				.service(routes::api::login)
		).await;
		
		let user = createUser(&db, username.to_owned(), label.to_owned()).await.unwrap();
		let data = LoginData { userId: Some(user.id) };
		let req = TestRequest::post()
			.uri(uri)
			.set_form(data)
			.to_request();
		
		let resp: LoginResponseData = test::call_and_read_body_json(&app, req).await;
		assert_eq!(resp.userId, Some(user.id));
		assert_eq!(resp.message, format!("Hello {}!", user.label))
	}
}
