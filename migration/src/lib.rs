pub use sea_orm_migration::prelude::*;

mod m20240813_000001_create_users;
mod m20240813_000002_create_game;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240813_000001_create_users::Migration),
            Box::new(m20240813_000002_create_game::Migration),
        ]
    }
}
