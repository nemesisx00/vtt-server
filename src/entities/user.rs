#![allow(dead_code, non_snake_case, non_upper_case_globals)]

use crate::entities::token;
use sea_orm::entity::prelude::*;
use serde::{
	Deserialize,
	Serialize,
};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "users")]
pub struct Model
{
	#[sea_orm(primary_key)]
	pub id: i64,
	pub label: String,
	pub name: Option<String>,
	pub avatar: Option<String>,
	pub description: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation
{
	#[sea_orm(has_many = "crate::entities::token::Entity")]
	Token,
}

impl Related<token::Entity> for Entity
{
	fn to() -> RelationDef
	{
		Relation::Token.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}
