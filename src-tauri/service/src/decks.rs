use ::entity;
use sea_orm::*;

pub struct DeckService;

impl DeckService {
    pub async fn find_all_decks<'a, C: ConnectionTrait>(
        db: &'a C,
    ) -> Result<Vec<entity::decks::Model>, DbErr> {
        entity::decks::Entity::find().all(db).await
    }

    pub async fn find_last_deck<'a, C: ConnectionTrait>(
        db: &'a C,
    ) -> Result<Option<entity::decks::Model>, DbErr> {
        entity::decks::Entity::find()
            .order_by_desc(entity::decks::Column::Id)
            .one(db)
            .await
    }

    pub async fn create_deck<'a, C: ConnectionTrait>(
        db: &'a C,
        form_data: entity::decks::Model,
    ) -> Result<entity::decks::ActiveModel, DbErr> {
        entity::decks::ActiveModel {
            name: Set(form_data.name.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_deck_by_id<'a, C: ConnectionTrait>(
        db: &'a C,
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

    pub async fn delete_deck<'a, C: ConnectionTrait>(
        db: &'a C,
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
