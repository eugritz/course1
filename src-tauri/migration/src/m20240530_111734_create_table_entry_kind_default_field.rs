use entity::entry_kind_default_field;
use entity::entry_kind_fields;
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
                    .table(entry_kind_default_field::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(
                            entry_kind_default_field::Column::EntryKindId,
                        )
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                entry_kind_default_field::Entity,
                                entry_kind_default_field::Column::EntryKindId,
                            )
                            .to(entry_kinds::Entity, entry_kinds::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(
                            entry_kind_default_field::Column::EntryKindFieldId,
                        )
                            .integer()
                            .not_null()
                            .unique_key()
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                entry_kind_default_field::Entity,
                                entry_kind_default_field::Column::EntryKindFieldId,
                            )
                            .to(
                                entry_kind_fields::Entity,
                                entry_kind_fields::Column::Id,
                            )
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(entry_kind_default_field::Entity)
                    .to_owned(),
            )
            .await
    }
}
