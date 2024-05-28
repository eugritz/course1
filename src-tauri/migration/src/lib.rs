pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20240528_130835_create_table_entry_kinds;
mod m20240528_150752_add_entry_kinds;
mod m20240528_210622_add_deck;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20240528_130835_create_table_entry_kinds::Migration),
            Box::new(m20240528_150752_add_entry_kinds::Migration),
            Box::new(m20240528_210622_add_deck::Migration),
        ]
    }
}
