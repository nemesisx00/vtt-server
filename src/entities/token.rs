#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::{
	api::structs::Point2D,
	entities::user,
};
use sea_orm::entity::prelude::*;
use serde::{
	Deserialize,
	Serialize,
};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "tokens")]
pub struct Model
{
	#[sea_orm(primary_key)]
	pub id: i64,
	pub label: String,
	pub avatar: String,
	pub x: f64,
	pub y: f64,
	pub ownerId: i64,
}

impl Model
{
	fn point(&self) -> Point2D
	{
		return Point2D { x: self.x, y: self.y };
	}
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation
{
	#[sea_orm(
		from = "Column::OwnerId",
		belongs_to = "crate::entities::user::Entity",
		to = "crate::entities::user::Column::Id"
	)]
	User,
}

impl Related<user::Entity> for Entity
{
	fn to() -> RelationDef
	{
		Relation::User.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}
