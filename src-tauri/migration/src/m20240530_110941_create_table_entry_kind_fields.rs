use entity::entry_kind_fields;
use entity::entry_kinds;
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
                    .table(entry_kind_fields::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(entry_kind_fields::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(entry_kind_fields::Column::EntryKindId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(entry_kind_fields::Column::Order)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                entry_kind_fields::Entity,
                                entry_kind_fields::Column::EntryKindId,
                            )
                            .to(entry_kinds::Entity, entry_kinds::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(entry_kind_fields::Column::Name)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(entry_kind_fields::Column::Desc)
                            .string()
                            .default("")
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(entry_kind_fields::Column::Type)
                            .string()
                            .default("ANY")
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        db.execute(
            builder.build(
                &Index::create()
                    .name("entry_kind_fields_entry_kind_id_idx")
                    .table(entry_kind_fields::Entity)
                    .col(entry_kind_fields::Column::EntryKindId)
                    .col(entry_kind_fields::Column::Name)
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
                Table::drop().table(entry_kind_fields::Entity).to_owned(),
            )
            .await
    }
}
