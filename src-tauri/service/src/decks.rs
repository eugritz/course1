use ::entity::decks;
use sea_orm::*;

pub struct DeckService;

impl DeckService {
    pub async fn find_all_decks<'a, C: ConnectionTrait>(
        db: &'a C,
    ) -> Result<Vec<decks::Model>, DbErr> {
        decks::Entity::find().all(db).await
    }

    pub async fn find_last_deck<'a, C: ConnectionTrait>(
        db: &'a C,
    ) -> Result<Option<decks::Model>, DbErr> {
        decks::Entity::find()
            .order_by_desc(decks::Column::Id)
            .one(db)
            .await
    }

    pub async fn create_deck<'a, C: ConnectionTrait>(
        db: &'a C,
        form_data: decks::Model,
    ) -> Result<decks::ActiveModel, DbErr> {
        decks::ActiveModel {
            name: Set(form_data.name.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_deck_by_id<'a, C: ConnectionTrait>(
        db: &'a C,
        id: i32,
        form_data: decks::Model,
    ) -> Result<decks::Model, DbErr> {
        let deck: decks::ActiveModel = decks::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find deck".to_owned()))
            .map(Into::into)?;

        decks::ActiveModel {
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
        let deck: decks::ActiveModel = decks::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find deck".to_owned()))
            .map(Into::into)?;

        deck.delete(db).await
    }
}
