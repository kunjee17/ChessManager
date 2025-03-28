use anyhow::Result;
use sea_orm::{Database, DatabaseConnection};
pub async fn get_db_connection() -> Result<DatabaseConnection> {
    let db: DatabaseConnection = Database::connect(
            "sqlite://chessmanager.sqlite?mode=rwc",
        )
        .await?;
    Ok(db)
}
