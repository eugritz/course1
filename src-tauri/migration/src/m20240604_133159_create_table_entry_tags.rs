use entity::{entries, entry_tags, tags};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(entry_tags::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(entry_tags::Column::EntryId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                entry_tags::Entity,
                                entry_tags::Column::EntryId,
                            )
                            .to(entries::Entity, entries::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(entry_tags::Column::TagId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(entry_tags::Entity, entry_tags::Column::TagId)
                            .to(tags::Entity, tags::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .primary_key(
                        Index::create()
                            .table(entry_tags::Entity)
                            .col(entry_tags::Column::EntryId)
                            .col(entry_tags::Column::TagId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(entry_tags::Entity).to_owned())
            .await
    }
}
