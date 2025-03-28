//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "player")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub fide_id: String,
    pub rating: i32,
    pub title: String,
    pub rating_system: String,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::tournament_player::Entity")]
    TournamentPlayer,
}
impl Related<super::tournament_player::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TournamentPlayer.def()
    }
}
impl ActiveModelBehavior for ActiveModel {}
