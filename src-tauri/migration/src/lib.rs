pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20240528_210622_add_deck;
mod m20240530_110319_create_table_entry_kinds;
mod m20240530_110941_create_table_entry_kind_fields;
mod m20240530_111734_create_table_entry_kind_default_field;
mod m20240530_112755_add_entry_kinds;
mod m20240603_121611_create_table_entries;
mod m20240603_123859_create_table_entry_field_values;
mod m20240604_001716_create_table_cards;
mod m20240604_002431_add_cards;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20240528_210622_add_deck::Migration),
            Box::new(m20240530_110319_create_table_entry_kinds::Migration),
            Box::new(m20240530_110941_create_table_entry_kind_fields::Migration),
            Box::new(m20240530_111734_create_table_entry_kind_default_field::Migration),
            Box::new(m20240530_112755_add_entry_kinds::Migration),
            Box::new(m20240603_121611_create_table_entries::Migration),
            Box::new(m20240603_123859_create_table_entry_field_values::Migration),
            Box::new(m20240604_001716_create_table_cards::Migration),
            Box::new(m20240604_002431_add_cards::Migration),
        ]
    }
}
