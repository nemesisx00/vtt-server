#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::{
	api::structs::*,
	entities::{
		user,
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
use sea_orm::{
	DatabaseConnection,
	EntityTrait,
};

#[get("/")]
pub async fn root() -> impl Responder
{
	return HttpResponse::Ok().body("Hello VTT World!");
}

#[post("/login")]
pub async fn login(data: Form<LoginData>, db: Data<DatabaseConnection>) -> impl Responder
{
	let mut resp = ResponseData
	{
		message: "No User found!".to_owned(),
		..Default::default()
	};
	
	if let Ok(Some(u)) = user::Entity::find_by_id(data.userId).one(db.get_ref()).await
	{
		resp.payload = Some(u.id);
		resp.message = format!("Hello {}!", u.label);
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
			ResponseData,
		},
		database::tests::createTestDatabase,
		entities::UserActive,
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
	use sea_orm::{
		ActiveModelTrait,
		Set,
	};
	
	#[actix_web::test]
	async fn test_login_invalid()
	{
		let uri = "/login";
		let expectedId = 1;
		
		let db = createTestDatabase().await.unwrap();
		let app = test::init_service(
			App::new()
				.app_data(Data::new(db.to_owned()))
				.service(routes::api::root::login)
		).await;
		
		let data = LoginData { userId: expectedId };
		let req = TestRequest::post()
			.uri(uri)
			.set_form(data)
			.to_request();
		
		let resp: ResponseData<i64> = test::call_and_read_body_json(&app, req).await;
		assert_eq!(resp.payload, None);
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
				.service(routes::api::root::login)
		).await;
		
		let active = UserActive
		{
			username: Set(username.to_owned()),
			label: Set(label.to_owned()),
			..Default::default()
		};
		
		let user = active.insert(&db).await.unwrap();
		let data = LoginData { userId: user.id };
		let req = TestRequest::post()
			.uri(uri)
			.set_form(data)
			.to_request();
		
		let resp: ResponseData<i64> = test::call_and_read_body_json(&app, req).await;
		assert_eq!(resp.payload, Some(user.id));
		assert_eq!(resp.message, format!("Hello {}!", user.label))
	}
}
