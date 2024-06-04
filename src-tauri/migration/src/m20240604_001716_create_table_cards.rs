use entity::cards;
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
                    .table(cards::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(cards::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(cards::Column::EntryKindId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(cards::Entity, cards::Column::EntryKindId)
                            .to(entry_kinds::Entity, entry_kinds::Column::Id),
                    )
                    .col(
                        ColumnDef::new(cards::Column::Name).string().not_null(),
                    )
                    .col(
                        ColumnDef::new(cards::Column::SourceFront)
                            .blob(BlobSize::Medium)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(cards::Column::SourceBack)
                            .blob(BlobSize::Medium)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(cards::Column::SourceStyle)
                            .blob(BlobSize::Medium)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(cards::Column::Immutable)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(cards::Entity).to_owned())
            .await
    }
}
