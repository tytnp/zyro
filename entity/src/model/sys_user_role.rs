//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "sys_user_role")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: Option<i32>,
    #[sea_orm(primary_key, auto_increment = false)]
    pub role_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::sys_role::Entity",
        from = "Column::RoleId",
        to = "super::sys_role::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    SysRole,
    #[sea_orm(
        belongs_to = "super::sys_user::Entity",
        from = "Column::UserId",
        to = "super::sys_user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    SysUser,
}

impl Related<super::sys_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SysRole.def()
    }
}

impl Related<super::sys_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SysUser.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
