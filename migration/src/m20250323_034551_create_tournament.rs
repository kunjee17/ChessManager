use sea_orm_migration::{prelude::*, schema::*, sea_orm::{EnumIter, Iterable}};
#[derive(DeriveMigrationName)]
pub struct Migration;
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Tournament::Table)
                    .if_not_exists()
                    .col(pk_auto(Tournament::Id))
                    .col(string(Tournament::Name))
                    .col(string(Tournament::Description))
                    .col(timestamp(Tournament::StartDate))
                    .col(timestamp(Tournament::EndDate))
                    .col(integer(Tournament::TotalRounds))
                    .col(integer(Tournament::CurrentRound))
                    .col(
                        enumeration(
                            Tournament::Status,
                            Alias::new("status"),
                            Status::iter(),
                        ),
                    )
                    .col(integer(Tournament::MaxPlayers))
                    .col(integer(Tournament::CurrentPlayers))
                    .col(string(Tournament::TimeControl))
                    .col(string(Tournament::Format))
                    .col(string(Tournament::TiebreakMethods))
                    .col(string(Tournament::ColorAllocation))
                    .col(string(Tournament::PairingAlgorithm))
                    .col(
                        timestamp(Tournament::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp(Tournament::UpdatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Player::Table)
                    .if_not_exists()
                    .col(pk_auto(Player::Id))
                    .col(string(Player::Name))
                    .col(string(Player::FideId))
                    .col(integer(Player::Rating))
                    .col(string(Player::Title))
                    .col(string(Player::RatingSystem))
                    .col(timestamp(Player::CreatedAt))
                    .col(timestamp(Player::UpdatedAt))
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(TournamentPlayer::Table)
                    .if_not_exists()
                    .col(pk_auto(TournamentPlayer::Id))
                    .col(integer(TournamentPlayer::TournamentId).not_null())
                    .col(integer(TournamentPlayer::PlayerId).not_null())
                    .col(integer(TournamentPlayer::StartingRank))
                    .col(integer(TournamentPlayer::CurrentRank))
                    .col(decimal(TournamentPlayer::Score))
                    .col(decimal(TournamentPlayer::Buchholz))
                    .col(decimal(TournamentPlayer::SonnebornBerger))
                    .col(timestamp(TournamentPlayer::CreatedAt))
                    .col(timestamp(TournamentPlayer::UpdatedAt))
                    .foreign_key(
                        &mut ForeignKey::create()
                            .name("fk_tournament_player_tournament")
                            .from(
                                TournamentPlayer::Table,
                                TournamentPlayer::TournamentId,
                            )
                            .to(Tournament::Table, Tournament::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .to_owned(),
                    )
                    .foreign_key(
                        &mut ForeignKey::create()
                            .name("fk_tournament_player_player")
                            .from(TournamentPlayer::Table, TournamentPlayer::PlayerId)
                            .to(Player::Table, Player::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .to_owned(),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Match::Table)
                    .if_not_exists()
                    .col(pk_auto(Match::Id))
                    .col(integer(Match::TournamentId).not_null())
                    .col(integer(Match::Round))
                    .col(integer(Match::WhitePlayerId).not_null())
                    .col(integer(Match::BlackPlayerId).not_null())
                    .col(string(Match::Result))
                    .col(string(Match::Pgn))
                    .col(integer(Match::WhiteTimeLeft))
                    .col(integer(Match::BlackTimeLeft))
                    .col(timestamp(Match::CreatedAt))
                    .col(timestamp(Match::UpdatedAt))
                    .foreign_key(
                        &mut ForeignKey::create()
                            .name("fk_match_tournament")
                            .from(Match::Table, Match::TournamentId)
                            .to(Tournament::Table, Tournament::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .to_owned(),
                    )
                    .foreign_key(
                        &mut ForeignKey::create()
                            .name("fk_match_white_player")
                            .from(Match::Table, Match::WhitePlayerId)
                            .to(Player::Table, Player::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .to_owned(),
                    )
                    .foreign_key(
                        &mut ForeignKey::create()
                            .name("fk_match_black_player")
                            .from(Match::Table, Match::BlackPlayerId)
                            .to(Player::Table, Player::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .to_owned(),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Setting::Table)
                    .if_not_exists()
                    .col(pk_auto(Setting::Id))
                    .col(string(Setting::Key))
                    .col(string(Setting::Value))
                    .col(string(Setting::Category))
                    .col(timestamp(Setting::CreatedAt))
                    .col(timestamp(Setting::UpdatedAt))
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Setting::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Match::Table).to_owned()).await?;
        manager
            .drop_table(Table::drop().table(TournamentPlayer::Table).to_owned())
            .await?;
        manager.drop_table(Table::drop().table(Player::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Tournament::Table).to_owned()).await?;
        Ok(())
    }
}
#[derive(DeriveIden)]
enum Tournament {
    Table,
    Id,
    Name,
    Description,
    StartDate,
    EndDate,
    TotalRounds,
    CurrentRound,
    Status,
    MaxPlayers,
    CurrentPlayers,
    TimeControl,
    Format,
    TiebreakMethods,
    ColorAllocation,
    PairingAlgorithm,
    CreatedAt,
    UpdatedAt,
}
#[derive(Iden, EnumIter)]
pub enum Status {
    #[iden = "Active"]
    Active,
    #[iden = "Scheduled"]
    Scheduled,
    #[iden = "Completed"]
    Completed,
    #[iden = "Cancelled"]
    Cancelled,
}
#[derive(DeriveIden)]
enum Player {
    Table,
    Id,
    Name,
    FideId,
    Rating,
    Title,
    RatingSystem,
    CreatedAt,
    UpdatedAt,
}
#[derive(DeriveIden)]
enum TournamentPlayer {
    Table,
    Id,
    TournamentId,
    PlayerId,
    StartingRank,
    CurrentRank,
    Score,
    Buchholz,
    SonnebornBerger,
    CreatedAt,
    UpdatedAt,
}
#[derive(DeriveIden)]
enum Match {
    Table,
    Id,
    TournamentId,
    Round,
    WhitePlayerId,
    BlackPlayerId,
    Result,
    Pgn,
    WhiteTimeLeft,
    BlackTimeLeft,
    CreatedAt,
    UpdatedAt,
}
#[derive(DeriveIden)]
enum Setting {
    Table,
    Id,
    Key,
    Value,
    Category,
    CreatedAt,
    UpdatedAt,
}
