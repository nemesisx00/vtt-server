#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::{
	api::{
		structs::{
			CreateUserData,
			DeleteUserData,
			UpdateUserData,
			ResponseData,
		},
	},
	entities::{
		user,
		User,
		UserActive,
	},
};
use std::error::Error;
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
	DatabaseConnection,
	EntityTrait, ModelTrait,
};
use log::{
	error,
	info,
};

#[post("/user/delete")]
pub async fn userDelete(data: Form<DeleteUserData>, db: Data<DatabaseConnection>) -> impl Responder
{
	let response = match user::Entity::find_by_id(data.userId).one(db.get_ref()).await
	{
		Ok(opt) => match opt
		{
			Some(user) => match user.delete(db.get_ref()).await
			{
				Ok(result) => ResponseData::<bool> { payload: Some(true), message: format!("Deleted {} users!", result.rows_affected) },
				Err(e) => {
					error!("Error deleting user {}: {:?}", data.userId, e);
					ResponseData::<bool> { payload: Some(false), message: "Failed to delete user!".to_owned() }
				}
			},
			None => ResponseData::<bool> { payload: Some(true), message: "No user to delete!".to_owned() }
		},
		Err(e) => {
			error!("Error finding user {}: {:?}", data.userId, e);
			ResponseData::<bool> { payload: Some(false), message: "Failed to find user to delete!".to_owned() }
		}
	};
	
	let json = serde_json::to_string(&response).unwrap();
	return HttpResponse::Ok().body(json);
}

#[post("/user/list")]
pub async fn userList(db: Data<DatabaseConnection>) -> impl Responder
{
	//TODO: authenticate
	
	let mut users = vec![];
	match user::Entity::find().all(db.get_ref()).await
	{
		Ok(result) => users = result,
		Err(e) => error!("Error retrieving all users: {}", e),
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
		let active = UserActive
		{
			username: Set(data.username.to_owned()),
			label: Set(data.label.to_owned()),
			..Default::default()
		};
		
		response = match active.insert(db.get_ref()).await
		{
			Ok(user) => ResponseData { payload: Some(user), message: "User created successfully!".to_owned() },
			Err(e) => {
				error!("Error creating user: {}", e);
				ResponseData { message: "Failed to create user!".to_owned(), ..Default::default() }
			},
		};
	}
	
	let json = serde_json::to_string(&response).unwrap();
	return HttpResponse::Ok().body(json);
}

async fn doUserUpdate(user: User, data: &UpdateUserData, db: &DatabaseConnection) -> Result<User, Box<dyn Error>>
{
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
	
	let updated = active.update(db).await?;
	return Ok(updated);
}

#[post("/user/update")]
pub async fn userUpdate(data: Form<UpdateUserData>, db: Data<DatabaseConnection>) -> impl Responder
{
	//TODO: authenticate
	
	let result = user::Entity::find_by_id(data.userId).one(db.get_ref()).await;
	
	let response = match result
	{
		Ok(opt) => match opt
		{
			Some(user) => {
				let updated = doUserUpdate(user, &data, db.get_ref()).await.unwrap();
				ResponseData { payload: Some(updated.to_owned()), message: format!("User '{}' updated!", updated.username.to_owned()) }
			},
			None => ResponseData { message: format!("No user found matching user ID '{}'!", data.userId.to_owned()), ..Default::default() },
		},
		Err(e) => ResponseData { message: format!("Error updating user! {}", e), ..Default::default() },
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
			DeleteUserData,
			ResponseData,
			UpdateUserData,
		},
		database::tests::createTestDatabase,
		entities::{
			user,
			User,
			UserActive,
		},
		routes::admin::user::*,
	};
	use actix_web::{
		test,
		web::{
			self,
			Data,
		},
		App,
	};
	use sea_orm::{
		ActiveModelTrait,
		EntityTrait,
		Set,
	};
	
	macro_rules! createApp {
		($db:ident, $service:ident) => {
			test::init_service(
				App::new()
					.app_data(Data::new($db.to_owned()))
					.service(
						web::scope("/admin")
							.service($service)
					)
			).await
		};
	}
	
	macro_rules! createRequest {
		($uri:expr) => {
			test::TestRequest::post()
				.uri($uri)
				.to_request()
		};
		($uri:expr, $data:ident) => {
			test::TestRequest::post()
				.uri($uri)
				.set_form($data)
				.to_request()
		}
	}
	
	#[actix_web::test]
	async fn test_userDelete_notFound()
	{
		let db = createTestDatabase().await.unwrap();
		let app = createApp!(db, userDelete);
		
		assert_eq!(user::Entity::find().all(&db).await.unwrap().len(), 0);
		
		let data = DeleteUserData { userId: 1 };
		let req = createRequest!("/admin/user/delete", data);
		let resp: ResponseData<bool> = test::call_and_read_body_json(&app, req).await;
		let wasDeleted = resp.payload.unwrap();
		assert!(wasDeleted);
	}
	
	#[actix_web::test]
	async fn test_userDelete_found()
	{
		let db = createTestDatabase().await.unwrap();
		let app = createApp!(db, userDelete);
		
		let active = UserActive
		{
			username: Set("username".to_owned()),
			label: Set("label".to_owned()),
			..Default::default()
		};
		let user = active.insert(&db).await.unwrap();
		
		assert_eq!(user::Entity::find().all(&db).await.unwrap().len(), 1);
		
		let data = DeleteUserData { userId: user.id };
		let req = createRequest!("/admin/user/delete", data);
		let resp: ResponseData<bool> = test::call_and_read_body_json(&app, req).await;
		let wasDeleted = resp.payload.unwrap();
		assert!(wasDeleted);
		
		assert_eq!(user::Entity::find().all(&db).await.unwrap().len(), 0);
	}
	
	#[actix_web::test]
	async fn test_userList()
	{
		let db = createTestDatabase().await.unwrap();
		let app = createApp!(db, userList);
		
		let active = UserActive
		{
			username: Set("username".to_owned()),
			label: Set("label".to_owned()),
			..Default::default()
		};
		
		let user = active.insert(&db).await.unwrap();
		let req = createRequest!("/admin/user/list");
		let resp: Vec<User> = test::call_and_read_body_json(&app, req).await;
		assert_eq!(resp.len(), 1);
		assert_eq!(resp[0], user);
	}
	
	#[actix_web::test]
	async fn test_userNew()
	{
		let db = createTestDatabase().await.unwrap();
		let app = createApp!(db, userNew);
		
		assert_eq!(user::Entity::find().all(&db).await.unwrap().len(), 0);
		
		let username = "username".to_owned();
		let label = "label".to_owned();
		
		let data = CreateUserData
		{
			username: username.to_owned(),
			label: label.to_owned(),
		};
		
		let req = createRequest!("/admin/user/new", data);
		let resp: ResponseData<User> = test::call_and_read_body_json(&app, req).await;
		let user = resp.payload.unwrap();
		
		assert_eq!(user.id, 1);
		assert_eq!(user.username, username);
		assert_eq!(user.label, label);
		
		let users = user::Entity::find().all(&db).await.unwrap();
		assert_eq!(user, users[0]);
	}
	
	#[actix_web::test]
	async fn test_userUpdate()
	{
		let db = createTestDatabase().await.unwrap();
		let app = createApp!(db, userUpdate);
		
		assert_eq!(user::Entity::find().all(&db).await.unwrap().len(), 0);
		
		let username = "username".to_owned();
		let label = "label".to_owned();
		let label2 = "Something different".to_owned();
		
		let active = UserActive
		{
			username: Set(username.to_owned()),
			label: Set(label.to_owned()),
			..Default::default()
		};
		
		let user = active.insert(&db).await.unwrap();
		
		assert_eq!(user::Entity::find().all(&db).await.unwrap().len(), 1);
		
		let data = UpdateUserData
		{
			userId: user.id.to_owned(),
			label: label2.to_owned(),
			..Default::default()
		};
		
		let req = createRequest!("/admin/user/update", data);
		let resp: ResponseData<User> = test::call_and_read_body_json(&app, req).await;
		let updated = resp.payload.unwrap();
		
		assert_ne!(updated, user);
		assert_eq!(updated.label, label2);
	}
}
