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
	pub UserId: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LoginResponseData
{
	pub UserId: Option<i64>,
	pub Message: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateUserResponse
{
	pub User: Option<User>,
	pub Message: String,
}
