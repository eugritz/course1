use ::entity::entry_kind_default_field;
use sea_orm::*;

pub struct EntryKindDefaultFieldService;

impl EntryKindDefaultFieldService {
    pub async fn set_entry_kind_default_field<'a, C: ConnectionTrait>(
        db: &'a C,
        entry_kind_id: i32,
        entry_kind_field_id: i32,
    ) -> Result<entry_kind_default_field::Model, DbErr> {
        entry_kind_default_field::Entity::find()
            .filter(
                entry_kind_default_field::Column::EntryKindId.eq(entry_kind_id),
            )
            .one(db)
            .await?
            .ok_or(DbErr::Custom(
                "Cannot find entry kind default field".to_owned(),
            ))?;

        entry_kind_default_field::ActiveModel {
            entry_kind_id: Set(entry_kind_id),
            entry_kind_field_id: Set(entry_kind_field_id),
        }
        .update(db)
        .await
    }
}
