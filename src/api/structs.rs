#![allow(dead_code, non_snake_case, non_upper_case_globals)]

use serde::{
	Deserialize,
	Serialize,
};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct LoginData
{
	pub UserId: Option<i64>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct LoginResponseData
{
	pub UserId: Option<i64>,
	pub Message: String,
}
