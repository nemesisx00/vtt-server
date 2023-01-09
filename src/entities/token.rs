#![allow(dead_code, non_snake_case, non_upper_case_globals)]

use crate::entities::user;
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
	pub label: Option<String>,
	pub avatar: String,
	pub ownerId: i64,
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
