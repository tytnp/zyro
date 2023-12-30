//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "sys_role")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: Option<String>,
    pub default_route: Option<String>,
    pub status: Option<bool>,
    pub created_at: Option<DateTimeUtc>,
    pub updated_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::sys_role_menu::Entity")]
    SysRoleMenu,
    #[sea_orm(has_many = "super::sys_user_role::Entity")]
    SysUserRole,
}

impl Related<super::sys_role_menu::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SysRoleMenu.def()
    }
}

impl Related<super::sys_user_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SysUserRole.def()
    }
}

impl Related<super::sys_menu::Entity> for Entity {
    fn to() -> RelationDef {
        super::sys_role_menu::Relation::SysMenu.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::sys_role_menu::Relation::SysRole.def().rev())
    }
}

impl Related<super::sys_user::Entity> for Entity {
    fn to() -> RelationDef {
        super::sys_user_role::Relation::SysUser.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::sys_user_role::Relation::SysRole.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
