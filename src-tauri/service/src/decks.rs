use ::entity;
use sea_orm::*;

pub struct DeckService;

impl DeckService {
    pub async fn find_all_decks(
        db: &DbConn,
    ) -> Result<Vec<entity::decks::Model>, DbErr> {
        entity::decks::Entity::find().all(db).await
    }

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

    pub async fn update_deck_by_id(
        db: &DbConn,
        id: i32,
        form_data: entity::decks::Model,
    ) -> Result<entity::decks::Model, DbErr> {
        let deck: entity::decks::ActiveModel =
            entity::decks::Entity::find_by_id(id)
                .one(db)
                .await?
                .ok_or(DbErr::Custom("Cannot find deck".to_owned()))
                .map(Into::into)?;

        entity::decks::ActiveModel {
            id: deck.id,
            name: Set(form_data.name.to_owned()),
        }
        .update(db)
        .await
    }

    pub async fn delete_deck(
        db: &DbConn,
        id: i32,
    ) -> Result<DeleteResult, DbErr> {
        let deck: entity::decks::ActiveModel =
            entity::decks::Entity::find_by_id(id)
                .one(db)
                .await?
                .ok_or(DbErr::Custom("Cannot find deck".to_owned()))
                .map(Into::into)?;

        deck.delete(db).await
    }
}
