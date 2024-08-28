use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

// https://github.com/SeaQL/sea-orm/blob/368b1126f73f47c7ec30fe523834f6a0962a193b/sea-orm-migration/src/schema.rs

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // User table
        // @todo verify all data saved is length-checked
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .col(pk_auto(User::Id))
                    .col(string_uniq(User::Username))
                    .col(string(User::PasswordHashAndSalt))
                    .col(string_null(User::Nickname))
                    .col(timestamp(User::CreationTime))
                    .col(timestamp(User::LastActiveTime))
                    .col(boolean(User::IsAdmin))
                    .col(string_null(User::Email))
                    .col(string_null(User::Avatar))
                    .col(string_null(User::ForgotPasswordRequest))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    Username,
    PasswordHashAndSalt,
    Nickname,
    CreationTime,
    LastActiveTime,
    IsAdmin,
    Email,
    Avatar,
    ForgotPasswordRequest,
}
