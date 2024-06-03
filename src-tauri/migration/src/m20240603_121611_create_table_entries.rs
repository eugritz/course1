use entity::decks;
use entity::entries;
use entity::entry_kinds;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(entries::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(entries::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(entries::Column::EntryKindId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(entries::Entity, entries::Column::EntryKindId)
                            .to(entry_kinds::Entity, entry_kinds::Column::Id),
                    )
                    .col(
                        ColumnDef::new(entries::Column::DeckId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(entries::Entity, entries::Column::DeckId)
                            .to(decks::Entity, decks::Column::Id),
                    )
                    .col(
                        ColumnDef::new(entries::Column::ColorTag)
                            .integer()
                            .default(0)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(entries::Column::Progress)
                            .float()
                            .default(0.0)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(entries::Column::CreatedAt)
                            .date()
                            .default(Expr::current_date())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(entries::Column::LastShownAt)
                            .timestamp(),
                    )
                    .col(
                        ColumnDef::new(entries::Column::NextShownAt)
                            .timestamp(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(entries::Entity).to_owned())
            .await
    }
}
