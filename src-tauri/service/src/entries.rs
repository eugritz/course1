use ::entity::entries;
use sea_orm::*;

use crate::entry_query_builder::EntryQueryBuilder;

pub struct EntryService;

impl EntryService {
    pub fn find<'a, C: ConnectionTrait + StreamTrait>(
        db: &'a C,
    ) -> EntryQueryBuilder<'a, C> {
        EntryQueryBuilder::new(db, entries::Entity::find())
    }

    pub async fn create_entry<'a, C: ConnectionTrait>(
        db: &'a C,
        form_data: entries::Model,
    ) -> Result<entries::ActiveModel, DbErr> {
        entries::ActiveModel {
            entry_kind_id: Set(form_data.entry_kind_id),
            deck_id: Set(form_data.deck_id),
            color_tag: Set(form_data.color_tag),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn delete_entry<'a, C: ConnectionTrait>(
        db: &'a C,
        form_data: entries::Model,
    ) -> Result<DeleteResult, DbErr> {
        entries::ActiveModel {
            id: Set(form_data.id),
            ..Default::default()
        }
        .delete(db)
        .await
    }
}
