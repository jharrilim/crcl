//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "login_oauth_info")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub login_id: i32,
    pub provider: String,
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expiry: Option<DateTime>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::logins::Entity",
        from = "Column::LoginId",
        to = "super::logins::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Logins,
}

impl Related<super::logins::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Logins.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
