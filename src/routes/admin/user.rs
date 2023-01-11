#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::{
	api::{
		args::parseArgs,
		structs::CreateUserResponse,
	},
	database::{
		findAllUsers,
		createUser,
	},
};
use actix_web::{
	post,
	web::Data,
	HttpResponse,
	Responder,
};
use sea_orm::DatabaseConnection;

#[post("/user/list")]
pub async fn userList(args: String, db: Data<DatabaseConnection>) -> impl Responder
{
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
pub async fn userNew(args: String, db: Data<DatabaseConnection>) -> impl Responder
{
	//TODO: if validate(args)
	let data = parseArgs(args.to_owned());
	let mut response = CreateUserResponse {
		user: None,
		message: format!("Invalid data! {}", args),
	};
	
	if data.contains_key("username") && !data["username"].is_empty() && data.contains_key("label") && !data["label"].is_empty()
	{
		response = match createUser(&db, data["username"].to_owned(), data["label"].to_owned()).await
		{
			Ok(user) => CreateUserResponse { user: Some(user), message: "User created successfully!".to_owned() },
			Err(e) => {
				println!("Error creating user: {}", e);
				CreateUserResponse { user: None, message: "Failed to create user!".to_owned() }
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
		api::structs::CreateUserResponse,
		database::{
			createTestDatabase,
			createUser,
			findAllUsers,
		},
		entities::User,
		routes,
	};
	use actix_web::{
		test,
		web::{
			self,
			Data,
		},
		App,
	};
	use serde::Serialize;
	
	#[derive(Clone, Debug, Serialize)]
	struct UserNewData
	{
		username: String,
		label: String,
	}
	
	#[actix_web::test]
	async fn test_userList()
	{
		let db = createTestDatabase().await.unwrap();
		let user = createUser(&db, "username".to_owned(), "label".to_owned()).await.unwrap();
		
		let app = test::init_service(
			App::new()
				.app_data(Data::new(db.to_owned()))
				.service(
					web::scope("/admin")
						.service(routes::admin::user::userList)
			)
		).await;
		
		let req = test::TestRequest::post().uri("/admin/user/list").to_request();
		
		let resp: Vec<User> = test::call_and_read_body_json(&app, req).await;
		assert_eq!(resp.len(), 1);
		assert_eq!(resp[0], user);
	}
	
	#[actix_web::test]
	async fn test_userNew()
	{
		let db = createTestDatabase().await.unwrap();
		
		let app = test::init_service(
			App::new()
				.app_data(Data::new(db.to_owned()))
				.service(
					web::scope("/admin")
						.service(routes::admin::user::userNew)
				)
		).await;
		
		let username = "username".to_owned();
		let label = "label".to_owned();
		
		let newData = UserNewData
		{
			username: username.to_owned(),
			label: label.to_owned(),
		};
		
		let req = test::TestRequest::post()
			.uri("/admin/user/new")
			.set_form(newData)
			.to_request();
		
		let resp: CreateUserResponse = test::call_and_read_body_json(&app, req).await;
		let user = resp.user.unwrap();
		
		assert_eq!(user.id, 1);
		assert_eq!(user.username, username);
		assert_eq!(user.label, label);
		
		let users = findAllUsers(&db).await.unwrap();
		assert_eq!(user, users[0]);
	}
}
