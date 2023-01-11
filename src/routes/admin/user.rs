#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::{
	api::{
		args::parseArgs,
		structs::CreateUserResponse,
	},
	database::{
		findAllUsers,
		createUser, getDatabaseConnection,
	},
};
use actix_web::{
	post,
	HttpResponse,
	Responder,
};

#[post("/user/list")]
pub async fn userList(args: String) -> impl Responder
{
	let db = getDatabaseConnection().await.unwrap();
	//println!("POST /admin/user/list: {}", args);
	//TODO: if validate(args)
	
	let _data = parseArgs(args);
	
	let mut users = vec![];
	match findAllUsers(&db).await
	{
		Ok(result) => users = result,
		Err(e) => println!("Error retrieving all users: {}", e),
	}
	println!("Did we get users? {:?}", users);
	let json = serde_json::to_string(&users).unwrap();
	
	return HttpResponse::Ok().body(json);
}

#[post("/user/new")]
pub async fn userNew(args: String) -> impl Responder
{
	let db = getDatabaseConnection().await.unwrap();
	//TODO: if validate(args)
	let data = parseArgs(args.to_owned());
	let mut response = CreateUserResponse {
		User: None,
		Message: format!("Invalid data! {}", args),
	};
	
	if data.contains_key("username") && !data["username"].is_empty() && data.contains_key("label") && !data["label"].is_empty()
	{
		response = match createUser(&db, data["username"].to_owned(), data["label"].to_owned()).await
		{
			Ok(user) => CreateUserResponse { User: Some(user), Message: "User created successfully!".to_owned() },
			Err(e) => {
				println!("Error creating user: {}", e);
				CreateUserResponse { User: None, Message: "Failed to create user!".to_owned() }
			},
		};
	}
	
	let json = serde_json::to_string(&response).unwrap();
	return HttpResponse::Ok().body(json);
}

#[cfg(test)]
mod tests
{
	use crate::{
		database::createTestDatabase,
		routes,
	};
	use actix_web::{
		test,
		web,
		App,
	};
	
	#[actix_web::test]
	async fn test_userList_success()
	{
		let _db = createTestDatabase().await.expect("Failed to create test database!");
		
		let app = test::init_service(
			App::new()
				.service(
					web::scope("/admin")
						.service(routes::admin::user::userList)
			)
		).await;
		
		let req = test::TestRequest::post().uri("/admin/user/list").to_request();
		let resp = test::call_service(&app, req).await;
		
		assert!(resp.status().is_success());
	}
}
