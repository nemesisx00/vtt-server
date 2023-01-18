#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::entities::{
	Token,
	User,
};
use serde::{
	Deserialize,
	Serialize,
};

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct ResponseData<T>
{
	pub payload: Option<T>,
	pub message: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateUserData
{
	pub username: String,
	pub label: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DeleteUserData
{
	pub userId: i64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LoginData
{
	pub userId: i64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Point2D
{
	pub x: f64,
	pub y: f64,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TokenUser
{
	pub token: Token,
	pub user: Option<User>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateUserData
{
	pub userId: i64,
	pub label: String,
	pub avatar: String,
	pub description: String,
}
