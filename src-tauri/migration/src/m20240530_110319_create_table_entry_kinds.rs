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
                    .table(entry_kinds::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(entry_kinds::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(entry_kinds::Column::Name)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(entry_kinds::Column::Default)
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
            .drop_table(Table::drop().table(entry_kinds::Entity).to_owned())
            .await
    }
}
