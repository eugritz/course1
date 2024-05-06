use ::entity;
use sea_orm::*;

pub struct DeckService;

impl DeckService {
    pub async fn create_deck(
        db: &DbConn,
        form_data: entity::decks::Model,
    ) -> Result<entity::decks::ActiveModel, DbErr> {
        entity::decks::ActiveModel {
            name: Set(form_data.name.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn find_all_decks(
        db: &DbConn,
    ) -> Result<Vec<entity::decks::Model>, DbErr> {
        entity::decks::Entity::find().all(db).await
    }
}
