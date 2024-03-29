//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_role_menu")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub role_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub menu_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::sys_menu::Entity",
        from = "Column::MenuId",
        to = "super::sys_menu::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    SysMenu,
    #[sea_orm(
        belongs_to = "super::sys_role::Entity",
        from = "Column::RoleId",
        to = "super::sys_role::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    SysRole,
}

impl Related<super::sys_menu::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SysMenu.def()
    }
}

impl Related<super::sys_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SysRole.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
