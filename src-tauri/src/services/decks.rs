use crate::entities;
use sea_orm::*;

pub struct DeckService;

impl DeckService {
    pub async fn create_deck(
        db: &DbConn,
        form_data: entities::decks::Model,
    ) -> Result<entities::decks::ActiveModel, DbErr> {
        entities::decks::ActiveModel {
            name: Set(form_data.name.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn find_all_decks(
        db: &DbConn,
    ) -> Result<Vec<entities::decks::Model>, DbErr> {
        entities::decks::Entity::find().all(db).await
    }
}
