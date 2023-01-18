#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::{
	api::structs::*,
	entities::{
		user,
		token,
	},
};
use actix_web::{
	get,
	post,
	web::{
		Data,
		Path,
	},
	HttpResponse,
	Responder,
};
use sea_orm::{
	DatabaseConnection,
	EntityTrait,
};

#[get("/token/{id}")]
pub async fn getToken(args: Path<i64>, db: Data<DatabaseConnection>) -> impl Responder
{
	let tokenId = args.into_inner();
	let mut resp = ResponseData
	{
		message: "No Token found!".to_owned(),
		..Default::default()
	};
	
	if let Ok(Some((t, u))) = token::Entity::find_by_id(tokenId)
		.find_also_related(user::Entity)
		.one(db.get_ref()).await
	{
		resp.payload = Some(TokenUser { token: t.to_owned(), user: u.to_owned() });
		resp.message = format!("Found Token {}!", t.id);
	}
	
	let respJson = serde_json::to_string(&resp).unwrap();
	return HttpResponse::Ok().body(respJson);
}

#[cfg(test)]
mod tests
{
	use crate::{
		api::structs::{
			ResponseData,
			TokenUser,
		},
		database::tests::createTestDatabase,
		entities::{
			TokenActive,
			UserActive,
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
	use sea_orm::{
		ActiveModelTrait,
		Set,
	};
	
	#[actix_web::test]
	async fn test_getToken_invalid()
	{
		let uri = "/token/0";
		
		let db = createTestDatabase().await.unwrap();
		let app = test::init_service(
			App::new()
				.app_data(Data::new(db.to_owned()))
				.service(routes::api::token::getToken)
		).await;
		
		let req = TestRequest::get()
			.uri(uri)
			.to_request();
		
		let resp: ResponseData<Option<TokenUser>> = test::call_and_read_body_json(&app, req).await;
		assert_eq!(resp.payload, None);
		assert_eq!(resp.message, "No Token found!".to_owned());
	}
	
	#[actix_web::test]
	async fn test_getToken_valid()
	{
		let username = "username";
		let label = "label";
		let avatar = "avatar";
		let x = 1.;
		let y = 2.;
		
		let db = createTestDatabase().await.unwrap();
		let app = test::init_service(
			App::new()
				.app_data(Data::new(db.to_owned()))
				.service(routes::api::token::getToken)
		).await;
		
		let active1 = UserActive
		{
			username: Set(username.to_owned()),
			label: Set(label.to_owned()),
			..Default::default()
		};
		let user = active1.insert(&db).await.unwrap();
		
		let active = TokenActive
		{
			label: Set(label.to_owned()),
			avatar: Set(avatar.to_owned()),
			x: Set(x.to_owned()),
			y: Set(y.to_owned()),
			ownerId: Set(user.id.to_owned()),
			..Default::default()
		};
		let token = active.insert(&db).await.unwrap();
		let uri = format!("/token/{}", token.id);
		let expected = TokenUser { token: token.to_owned(), user: Some(user.to_owned()) };
		
		let req = TestRequest::get()
			.uri(uri.as_ref())
			.to_request();
		
		let resp: ResponseData<TokenUser> = test::call_and_read_body_json(&app, req).await;
		assert_eq!(resp.payload, Some(expected));
		assert_eq!(resp.message, "Found Token 1!".to_owned());
	}
}
