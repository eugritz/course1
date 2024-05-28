use entity::entry_kinds;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{entity::*, QueryFilter};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        entry_kinds::Entity::insert_many([
            entry_kinds::ActiveModel {
                name: Set("Простая (с обратной картой)".to_owned()),
                default: Set(true),
                ..Default::default()
            },
            entry_kinds::ActiveModel {
                name: Set("Простая".to_owned()),
                default: Set(true),
                ..Default::default()
            },
        ])
        .exec(db)
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        entry_kinds::Entity::delete_many()
            .filter(
                Condition::any()
                    .add(entry_kinds::Column::Name.eq("Простая".to_owned()))
                    .add(
                        entry_kinds::Column::Name
                            .eq("Простая (с обратной картой)".to_owned()),
                    ),
            )
            .exec(db)
            .await?;

        Ok(())
    }
}
