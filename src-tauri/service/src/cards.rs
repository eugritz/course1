use ::entity::cards;
use sea_orm::*;

pub struct CardService;

impl CardService {
    pub async fn find_all_entry_kind_cards<'a, C: ConnectionTrait>(
        db: &'a C,
        entry_kind_id: i32,
    ) -> Result<Vec<cards::Model>, DbErr> {
        cards::Entity::find()
            .filter(cards::Column::EntryKindId.eq(entry_kind_id))
            .all(db)
            .await
    }

    pub async fn create_card<'a, C: ConnectionTrait>(
        db: &'a C,
        form_data: cards::Model,
    ) -> Result<cards::Model, DbErr> {
        cards::ActiveModel {
            entry_kind_id: Set(form_data.entry_kind_id),
            name: Set(form_data.name),
            source_front: Set(form_data.source_front),
            source_back: Set(form_data.source_back),
            source_style: Set(form_data.source_style),
            ..Default::default()
        }
        .insert(db)
        .await
    }
}
