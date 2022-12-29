#![allow(dead_code, non_snake_case, non_upper_case_globals)]

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
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

impl Related<crate::entities::User> for Entity
{
	fn to() -> RelationDef
	{
		Relation::User.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}
