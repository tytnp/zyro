//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "sys_dictionary_detail")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Option<i32>,
    pub dictionary_id: Option<i32>,
    pub label: Option<String>,
    pub value: Option<i32>,
    pub sort: Option<i32>,
    pub status: Option<bool>,
    pub created_at: Option<DateTimeUtc>,
    pub updated_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::sys_dictionary::Entity",
        from = "Column::DictionaryId",
        to = "super::sys_dictionary::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    SysDictionary,
}

impl Related<super::sys_dictionary::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SysDictionary.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}