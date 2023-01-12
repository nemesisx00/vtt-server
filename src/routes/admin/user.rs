#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::{
	api::{
		structs::{
			CreateUserData,
			UpdateUserData,
			ResponseData,
		},
	},
	database::{
		findAllUsers,
		findUserById,
		createUser,
	},
	entities::{
		UserActive,
	},
};
use actix_web::{
	post,
	web::{
		Data,
		Form,
	},
	HttpResponse,
	Responder,
};
use sea_orm::{
	Set,
	ActiveModelTrait,
	DatabaseConnection
};

#[post("/user/list")]
pub async fn userList(db: Data<DatabaseConnection>) -> impl Responder
{
	//TODO: authenticate
	
	let mut users = vec![];
	match findAllUsers(db.get_ref()).await
	{
		Ok(result) => users = result,
		Err(e) => println!("Error retrieving all users: {}", e),
	}
	
	let json = serde_json::to_string(&users).unwrap();
	
	return HttpResponse::Ok().body(json);
}

#[post("/user/new")]
pub async fn userNew(data: Form<CreateUserData>, db: Data<DatabaseConnection>) -> impl Responder
{
	//TODO: authenticate
	
	let mut response = ResponseData {
		message: format!("Invalid data! {:?}", data),
		..Default::default()
	};
	
	if !data.username.is_empty() && !data.label.is_empty()
	{
		response = match createUser(db.get_ref(), data.username.to_owned(), data.label.to_owned()).await
		{
			Ok(user) => ResponseData { payload: Some(user), message: "User created successfully!".to_owned() },
			Err(e) => {
				println!("Error creating user: {}", e);
				ResponseData { message: "Failed to create user!".to_owned(), ..Default::default() }
			},
		};
	}
	
	let json = serde_json::to_string(&response).unwrap();
	return HttpResponse::Ok().body(json);
}

#[post("/user/update")]
pub async fn userUpdate(data: Form<UpdateUserData>, db: Data<DatabaseConnection>) -> impl Responder
{
	//TODO: authenticate
	
	let result = findUserById(db.get_ref(), data.userId.to_owned()).await;
	
	let response = match result
	{
		Ok(opt) => match opt
		{
			Some(user) => {
				let mut active: UserActive = user.into();
				
				active.label = Set(data.label.to_owned());
				
				match data.avatar.is_empty()
				{
					true => active.avatar = Set(None),
					false => active.avatar = Set(Some(data.avatar.to_owned()))
				}
				
				match data.avatar.is_empty()
				{
					true => active.description = Set(None),
					false => active.description = Set(Some(data.description.to_owned()))
				}
				
				let updated = active.update(db.get_ref()).await.unwrap();
				
				ResponseData
				{
					payload: Some(updated.to_owned()),
					message: format!("User '{}' updated!", updated.username.to_owned()),
				}
			},
			
			None => ResponseData
			{
				message: format!("No user found matching user ID '{}'!", data.userId.to_owned()),
				..Default::default()
			},
		},
		
		Err(e) => ResponseData
		{
			message: format!("Error updating user! {}", e),
			..Default::default()
		},
	};
	
	//let user: UserActive 
	let json = serde_json::to_string(&response).unwrap();
	return HttpResponse::Ok().body(json);
}

#[cfg(test)]
mod tests
{
	use crate::{
		api::structs::{
			CreateUserData,
			ResponseData,
			UpdateUserData,
		},
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
		
		let newData = CreateUserData
		{
			username: username.to_owned(),
			label: label.to_owned(),
		};
		
		let req = test::TestRequest::post()
			.uri("/admin/user/new")
			.set_form(newData)
			.to_request();
		
		let resp: ResponseData<User> = test::call_and_read_body_json(&app, req).await;
		let user = resp.payload.unwrap();
		
		assert_eq!(user.id, 1);
		assert_eq!(user.username, username);
		assert_eq!(user.label, label);
		
		let users = findAllUsers(&db).await.unwrap();
		assert_eq!(user, users[0]);
	}
	
	#[actix_web::test]
	async fn test_userUpdate()
	{
		let db = createTestDatabase().await.unwrap();
		let app = test::init_service(
			App::new()
				.app_data(Data::new(db.to_owned()))
				.service(
					web::scope("/admin")
						.service(routes::admin::user::userUpdate)
				)
		).await;
		
		let username1 = "username".to_owned();
		let label = "label".to_owned();
		let label2 = "Something different".to_owned();
		
		let user = createUser(&db, username1.to_owned(), label.to_owned()).await.unwrap();
		
		let data = UpdateUserData
		{
			userId: user.id.to_owned(),
			label: label2.to_owned(),
			..Default::default()
		};
		
		let req = test::TestRequest::post()
			.uri("/admin/user/update")
			.set_form(data)
			.to_request();
		
		let resp: ResponseData<User> = test::call_and_read_body_json(&app, req).await;
		let updated = resp.payload.unwrap();
		
		assert_ne!(updated, user);
		assert_eq!(updated.label, label2);
	}
}
