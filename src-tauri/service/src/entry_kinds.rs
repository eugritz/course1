use ::entity;
use sea_orm::*;

pub struct EntryKindService;

impl EntryKindService {
    pub async fn find_all_entry_kinds(
        db: &DbConn,
    ) -> Result<Vec<entity::entry_kinds::Model>, DbErr> {
        entity::entry_kinds::Entity::find().all(db).await
    }

    pub async fn find_last_entry_kind(
        db: &DbConn,
    ) -> Result<Option<entity::entry_kinds::Model>, DbErr> {
        entity::entry_kinds::Entity::find()
            .order_by_desc(entity::entry_kinds::Column::Id)
            .one(db)
            .await
    }

    pub async fn find_entry_kind_by_id(
        db: &DbConn,
        id: i32,
    ) -> Result<Option<entity::entry_kinds::Model>, DbErr> {
        entity::entry_kinds::Entity::find_by_id(id).one(db).await
    }

    pub async fn create_entry_kind(
        db: &DbConn,
        form_data: entity::entry_kinds::Model,
    ) -> Result<entity::entry_kinds::ActiveModel, DbErr> {
        entity::entry_kinds::ActiveModel {
            name: Set(form_data.name.to_owned()),
            default: Set(form_data.default),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_entry_kind_by_id(
        db: &DbConn,
        id: i32,
        form_data: entity::entry_kinds::Model,
    ) -> Result<entity::entry_kinds::Model, DbErr> {
        let entry_kind: entity::entry_kinds::ActiveModel =
            entity::entry_kinds::Entity::find_by_id(id)
                .one(db)
                .await?
                .ok_or(DbErr::Custom("Cannot find entry kind".to_owned()))
                .map(Into::into)?;

        entity::entry_kinds::ActiveModel {
            id: entry_kind.id,
            name: Set(form_data.name.to_owned()),
            default: entry_kind.default,
        }
        .update(db)
        .await
    }

    pub async fn delete_entry_kind(
        db: &DbConn,
        id: i32,
    ) -> Result<DeleteResult, DbErr> {
        let entry_kind: entity::entry_kinds::ActiveModel =
            entity::entry_kinds::Entity::find_by_id(id)
                .one(db)
                .await?
                .ok_or(DbErr::Custom("Cannot find entry kind".to_owned()))
                .map(Into::into)?;

        entry_kind.delete(db).await
    }
}
