//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "team_result_to_user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub team_result_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::team_result::Entity",
        from = "Column::TeamResultId",
        to = "super::team_result::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    TeamResult,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
}

impl Related<super::team_result::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TeamResult.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
