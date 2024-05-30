use entity::entry_kind_default_field;
use entity::entry_kind_fields;
use entity::entry_kinds;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{entity::*, QueryFilter};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let entry_kinds = entry_kinds::Entity::insert_many([
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

        let entry_kind_fields = entry_kind_fields::Entity::insert_many([
            entry_kind_fields::ActiveModel {
                entry_kind_id: Set(entry_kinds.last_insert_id - 1),
                order: Set(1),
                name: Set("Зад".to_owned()),
                ..Default::default()
            },
            entry_kind_fields::ActiveModel {
                entry_kind_id: Set(entry_kinds.last_insert_id - 1),
                order: Set(2),
                name: Set("Перед".to_owned()),
                ..Default::default()
            },
            entry_kind_fields::ActiveModel {
                entry_kind_id: Set(entry_kinds.last_insert_id),
                order: Set(1),
                name: Set("Зад".to_owned()),
                ..Default::default()
            },
            entry_kind_fields::ActiveModel {
                entry_kind_id: Set(entry_kinds.last_insert_id),
                order: Set(2),
                name: Set("Перед".to_owned()),
                ..Default::default()
            },
        ])
        .exec(db)
        .await?;

        entry_kind_default_field::Entity::insert_many([
            entry_kind_default_field::ActiveModel {
                entry_kind_id: Set(entry_kinds.last_insert_id - 1),
                entry_kind_field_id: Set(entry_kind_fields.last_insert_id - 3),
            },
            entry_kind_default_field::ActiveModel {
                entry_kind_id: Set(entry_kinds.last_insert_id),
                entry_kind_field_id: Set(entry_kind_fields.last_insert_id - 1),
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
