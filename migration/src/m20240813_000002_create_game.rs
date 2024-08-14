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

        manager
            .create_type(schema.create_enum_from_active_enum::<PlayerSetupType>())
            .await?;
        manager
            .create_type(schema.create_enum_from_active_enum::<GameType>())
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Game::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Game::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Game::PlayerSetupType).custom(PlayerSetupType::name()))
                    .col(ColumnDef::new(Game::GameType).custom(GameType::name()))
                    .col(
                        ColumnDef::new(Game::Time)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Game::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Game::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Game::PlayerSetupType).custom(PlayerSetupType::name()))
                    .col(ColumnDef::new(Game::GameType).custom(GameType::name()))
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(OneVsOne::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OneVsOne::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(OneVsOne::PlayerOne).integer().not_null())
                    .col(ColumnDef::new(OneVsOne::PlayerTwo).integer().not_null())
                    .col(ColumnDef::new(OneVsOne::GameId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user-one_vs_one-player_one_id")
                            .from(OneVsOne::Table, OneVsOne::PlayerOne)
                            .to(User::Table, User::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user-one_vs_one-player_two_id")
                            .from(OneVsOne::Table, OneVsOne::PlayerTwo)
                            .to(User::Table, User::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-game-one_vs_one-game_id")
                            .from(OneVsOne::Table, OneVsOne::GameId)
                            .to(Game::Table, Game::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_type(Type::drop().name(PlayerSetupType::name()).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(GameType::name()).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Game::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(OneVsOne::Table).to_owned())
            .await
    }
}

// Enums
#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "player_setup_type")]
enum PlayerSetupType {
    #[sea_orm(string_value = "OneVsOne")]
    OneVsOne,
    #[sea_orm(string_value = "TwoVsTwo")]
    TwoVsTwo,
}

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
    PlayerSetupType,
    GameType,
}

#[derive(DeriveIden)]
enum OneVsOne {
    Table,
    Id,
    GameId,
    PlayerOne,
    PlayerTwo,
}
