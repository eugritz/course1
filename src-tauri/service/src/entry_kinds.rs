use ::entity::entry_kinds;
use sea_orm::*;

pub struct EntryKindService;

impl EntryKindService {
    pub async fn find_all_entry_kinds<'a, C: ConnectionTrait>(
        db: &'a C,
    ) -> Result<Vec<entry_kinds::Model>, DbErr> {
        entry_kinds::Entity::find().all(db).await
    }

    pub async fn find_last_entry_kind<'a, C: ConnectionTrait>(
        db: &'a C,
    ) -> Result<Option<entry_kinds::Model>, DbErr> {
        entry_kinds::Entity::find()
            .order_by_desc(entry_kinds::Column::Id)
            .one(db)
            .await
    }

    pub async fn find_entry_kind_by_id<'a, C: ConnectionTrait>(
        db: &'a C,
        id: i32,
    ) -> Result<Option<entry_kinds::Model>, DbErr> {
        entry_kinds::Entity::find_by_id(id).one(db).await
    }

    pub async fn create_entry_kind<'a, C: ConnectionTrait>(
        db: &'a C,
        form_data: entry_kinds::Model,
    ) -> Result<entry_kinds::ActiveModel, DbErr> {
        entry_kinds::ActiveModel {
            name: Set(form_data.name.to_owned()),
            immutable: Set(form_data.immutable),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_entry_kind_by_id<'a, C: ConnectionTrait>(
        db: &'a C,
        id: i32,
        form_data: entry_kinds::Model,
    ) -> Result<entry_kinds::Model, DbErr> {
        let entry_kind: entry_kinds::ActiveModel =
            entry_kinds::Entity::find_by_id(id)
                .one(db)
                .await?
                .ok_or(DbErr::Custom("Cannot find entry kind".to_owned()))
                .map(Into::into)?;

        entry_kinds::ActiveModel {
            id: entry_kind.id,
            name: Set(form_data.name.to_owned()),
            immutable: entry_kind.immutable,
        }
        .update(db)
        .await
    }

    pub async fn delete_entry_kind<'a, C: ConnectionTrait>(
        db: &'a C,
        id: i32,
    ) -> Result<DeleteResult, DbErr> {
        let entry_kind: entry_kinds::ActiveModel =
            entry_kinds::Entity::find_by_id(id)
                .one(db)
                .await?
                .ok_or(DbErr::Custom("Cannot find entry kind".to_owned()))
                .map(Into::into)?;

        entry_kind.delete(db).await
    }
}
