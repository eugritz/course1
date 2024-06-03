use entity::entries;
use entity::entry_field_values;
use entity::entry_kind_fields;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let builder = manager.get_database_backend();

        manager
            .create_table(
                Table::create()
                    .table(entry_field_values::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(entry_field_values::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(entry_field_values::Column::EntryId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                entry_field_values::Entity,
                                entry_field_values::Column::EntryId,
                            )
                            .to(entries::Entity, entries::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(
                            entry_field_values::Column::EntryFieldId,
                        )
                        .integer()
                        .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                entry_field_values::Entity,
                                entry_field_values::Column::EntryFieldId,
                            )
                            .to(
                                entry_kind_fields::Entity,
                                entry_kind_fields::Column::Id,
                            )
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(entry_field_values::Column::Value)
                            .string()
                            .default("")
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        db.execute(
            builder.build(
                &Index::create()
                    .name("entry_field_values_entry_id_idx")
                    .table(entry_field_values::Entity)
                    .col(entry_field_values::Column::EntryId)
                    .col(entry_field_values::Column::EntryFieldId)
                    .unique()
                    .to_owned(),
            ),
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop().table(entry_field_values::Entity).to_owned(),
            )
            .await
    }
}
