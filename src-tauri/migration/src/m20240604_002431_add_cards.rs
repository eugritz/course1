use entity::cards;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{entity::*, QueryFilter};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        cards::Entity::insert_many([
            cards::ActiveModel {
                entry_kind_id: Set(1),
                name: Set("Карточка 1".to_owned()),
                source_front: Set("{{Перед}}".as_bytes().to_vec()),
                source_back: Set("\
                    {{Перед}}\n\
                    \n\
                    <hr id=answer>\n\
                    \n\
                    {{Зад}}\
                "
                .as_bytes()
                .to_vec()),
                source_style: Set("\
                    .card {\n    \
                        font-family: arial;\n    \
                        font-size: 20px;\n    \
                        text-align: center;\n    \
                        color: black;\n    \
                        background-color: white;\n\
                    };\
                "
                .as_bytes()
                .to_vec()),
                immutable: Set(true),
                ..Default::default()
            },
            cards::ActiveModel {
                entry_kind_id: Set(2),
                name: Set("Карточка 1".to_owned()),
                source_front: Set("{{Перед}}".as_bytes().to_vec()),
                source_back: Set("\
                    {{Перед}}\n\
                    \n\
                    <hr id=answer>\n\
                    \n\
                    {{Зад}}\
                "
                .as_bytes()
                .to_vec()),
                source_style: Set("\
                    .card {\n    \
                        font-family: arial;\n    \
                        font-size: 20px;\n    \
                        text-align: center;\n    \
                        color: black;\n    \
                        background-color: white;\n\
                    };\
                "
                .as_bytes()
                .to_vec()),
                immutable: Set(true),
                ..Default::default()
            },
            cards::ActiveModel {
                entry_kind_id: Set(2),
                name: Set("Карточка 2".to_owned()),
                source_front: Set("{{Перед}}".as_bytes().to_vec()),
                source_back: Set("\
                    {{Перед}}\n\
                    \n\
                    <hr id=answer>\n\
                    \n\
                    {{Зад}}\
                "
                .as_bytes()
                .to_vec()),
                source_style: Set("\
                    .card {\n    \
                        font-family: arial;\n    \
                        font-size: 20px;\n    \
                        text-align: center;\n    \
                        color: black;\n    \
                        background-color: white;\n\
                    };\
                "
                .as_bytes()
                .to_vec()),
                immutable: Set(true),
                ..Default::default()
            },
        ])
        .exec(db)
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        cards::Entity::delete_many()
            .filter(
                Condition::any()
                    .add(cards::Column::Name.eq("Карточка 1"))
                    .add(cards::Column::Name.eq("Карточка 2")),
            )
            .exec(db)
            .await?;

        Ok(())
    }
}
