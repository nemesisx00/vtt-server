#![allow(dead_code, non_snake_case, non_upper_case_globals)]

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model
{
	#[sea_orm(primary_key)]
	pub id: i64,
	pub label: String,
	pub name: String,
	pub avatar: String,
	pub description: String,
	pub email: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation { }

impl ActiveModelBehavior for ActiveModel {}
