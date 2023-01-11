#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::entities::{
	User,
};
use serde::{
	Deserialize,
	Serialize,
};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LoginData
{
	pub userId: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LoginResponseData
{
	pub userId: Option<i64>,
	pub message: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateUserResponse
{
	pub user: Option<User>,
	pub message: String,
}
