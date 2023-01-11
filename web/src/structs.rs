#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use serde::{
	Deserialize,
	Serialize,
};

#[derive(Clone, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct User
{
	pub id: i64,
	pub label: String,
	pub name: Option<String>,
	pub avatar: Option<String>,
	pub description: Option<String>,
}
