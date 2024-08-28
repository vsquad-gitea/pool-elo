use sea_orm::{DbBackend, DeriveActiveEnum, EnumIter, Schema};
use sea_orm_migration::sea_orm::ActiveEnum;
use sea_orm_migration::{prelude::*, schema::*, sea_query::extension::postgres::Type};

use super::m20240813_000001_create_users::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let schema = Schema::new(DbBackend::Postgres);

        // Game type enum
        manager
            .create_type(schema.create_enum_from_active_enum::<GameType>())
            .await?;
        // Game table
        manager
            .create_table(
                Table::create()
                    .table(Game::Table)
                    .col(pk_auto(Game::Id))
                    .col(timestamp_with_time_zone(Game::Time))
                    .col(ColumnDef::new(Game::GameType).custom(GameType::name()))
                    .to_owned(),
            )
            .await?;
        // TeamResult table
        manager
            .create_table(
                Table::create()
                    .table(TeamResult::Table)
                    .col(pk_auto(TeamResult::Id))
                    .col(integer(TeamResult::Place))
                    .col(integer_null(TeamResult::Score))
                    .to_owned(),
            )
            .await?;
        // Game to TeamResult assoc
        manager
            .create_table(
                Table::create()
                    .table(GameToTeamResult::Table)
                    .col(integer(GameToTeamResult::GameId))
                    .col(integer(GameToTeamResult::TeamResultId))
                    .primary_key(
                        Index::create()
                            .name("pk-game_to_team_result")
                            .col(GameToTeamResult::GameId)
                            .col(GameToTeamResult::TeamResultId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-game_to_team_result-game_id")
                            .from(GameToTeamResult::Table, GameToTeamResult::GameId)
                            .to(Game::Table, Game::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-game_to_team_result-team_result_id")
                            .from(GameToTeamResult::Table, GameToTeamResult::TeamResultId)
                            .to(TeamResult::Table, TeamResult::Id),
                    )
                    .to_owned(),
            )
            .await?;
        // TeamResult to User assoc
        manager
            .create_table(
                Table::create()
                    .table(TeamResultToUser::Table)
                    .col(integer(TeamResultToUser::TeamResultId))
                    .col(integer(TeamResultToUser::UserId))
                    .primary_key(
                        Index::create()
                            .name("pk-team_result_to_user")
                            .col(TeamResultToUser::TeamResultId)
                            .col(TeamResultToUser::UserId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-team_result_to_user-team_result_id")
                            .from(TeamResultToUser::Table, TeamResultToUser::TeamResultId)
                            .to(TeamResult::Table, TeamResult::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-team_result_to_user-user_id")
                            .from(TeamResultToUser::Table, TeamResultToUser::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables in reverse
        manager
            .drop_table(Table::drop().table(TeamResultToUser::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(GameToTeamResult::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(TeamResult::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Game::Table).to_owned())
            .await?;
        // Drop types
        manager
            .drop_type(Type::drop().name(GameType::name()).to_owned())
            .await
    }
}

// Enums

#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "game_type")]
enum GameType {
    #[sea_orm(string_value = "TableTennis")]
    TableTennis,
    #[sea_orm(string_value = "Pool")]
    Pool,
    #[sea_orm(string_value = "PickleBall")]
    PickleBall,
}

// Tables
#[derive(DeriveIden)]
enum Game {
    Table,
    Id,
    Time,
    GameType,
}

#[derive(DeriveIden)]
enum TeamResult {
    Table,
    Id,
    Place,
    Score,
}

// Assoc
#[derive(DeriveIden)]
enum GameToTeamResult {
    Table,
    GameId,
    TeamResultId,
}

#[derive(DeriveIden)]
enum TeamResultToUser {
    Table,
    TeamResultId,
    UserId,
}
