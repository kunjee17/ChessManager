use super::types::TournamentForm;
use crate::{db::db::get_db_connection, entity::tournament};
use anyhow::Result;
use sea_orm::{entity::*, query::*};
pub async fn get_active_tournaments() -> Result<Vec<tournament::Model>> {
    let db = get_db_connection().await?;
    let tournaments = tournament::Entity::find()
        .filter(tournament::Column::Status.eq("Active"))
        .all(&db)
        .await?;
    Ok(tournaments)
}
pub async fn get_past_tournaments() -> Result<Vec<tournament::Model>> {
    let db = get_db_connection().await?;
    let tournaments = tournament::Entity::find()
        .filter(tournament::Column::Status.eq("Completed"))
        .all(&db)
        .await?;
    Ok(tournaments)
}
pub async fn get_upcoming_tournaments() -> Result<Vec<tournament::Model>> {
    let db = get_db_connection().await?;
    let tournaments = tournament::Entity::find()
        .filter(tournament::Column::Status.eq("Scheduled"))
        .all(&db)
        .await?;
    Ok(tournaments)
}
pub async fn save_tournament(tournament: TournamentForm) -> Result<tournament::Model> {
    let db = get_db_connection().await?;
    let tournament = tournament.to_active_model();
    let tournament = tournament.save(&db).await?;
    let tournament_model = tournament.try_into_model()?;
    Ok(tournament_model)
}
pub async fn delete_tournament(id: i32) -> Result<()> {
    let db = get_db_connection().await?;
    tournament::Entity::delete_by_id(id).exec(&db).await?;
    Ok(())
}
