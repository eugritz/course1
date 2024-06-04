use ::entity::tags;
use sea_orm::*;

pub struct TagService;

impl TagService {
    pub async fn find_all_tags<'a, C: ConnectionTrait>(
        db: &'a C,
    ) -> Result<Vec<tags::Model>, DbErr> {
        tags::Entity::find().all(db).await
    }

    pub async fn create_tags_if_abscent<'a, C: ConnectionTrait>(
        db: &'a C,
        tags: Vec<String>,
    ) -> Result<(), DbErr> {
        for tag in tags {
            let result = tags::ActiveModel {
                name: Set(tag),
                ..Default::default()
            }
            .insert(db)
            .await;

            match result {
                Ok(_) | Err(DbErr::RecordNotInserted) => {}
                Err(err) => return Err(err),
            }
        }

        Ok(())
    }

    pub async fn delete_orphan_tags<'a, C: ConnectionTrait>(
        db: &'a C,
    ) -> Result<(), DbErr> {
        let query =
            "DELETE FROM \"tags\" \
            WHERE \"tags\".\"id\" IN (\
                SELECT \"tags\".\"id\" \
                FROM \"tags\" \
                LEFT JOIN \"entry_tags\" ON \"tags\".\"id\" = \"entry_tags\".\"tag_id\" \
                GROUP BY \"tags\".\"id\", \"entry_tags\".\"tag_id\" \
                HAVING count(\"entry_tags\".\"tag_id\") = 0\
            );";

        db.execute(Statement::from_string(DatabaseBackend::Sqlite, query))
            .await
            .map(|_| ())
    }
}
