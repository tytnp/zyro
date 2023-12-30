//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "sys_api")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Option<i32>,
    pub path: Option<String>,
    pub description: Option<String>,
    pub api_group: Option<String>,
    pub method: Option<String>,
    pub status: Option<bool>,
    pub created_at: Option<DateTimeUtc>,
    pub updated_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
