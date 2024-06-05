use ::entity::{entry_tags, tags};
use sea_orm::*;

fn escape(s: String) -> String {
    let mut escaped = "".to_string();
    for ch in s.chars() {
        match ch {
            '\'' => escaped.push_str("''"),
            ch => escaped.push(ch),
        }
    }
    escaped
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
enum QueryAs {
    Name,
}

pub struct EntryTagsService;

impl EntryTagsService {
    pub async fn get_entry_tags<'a, C: ConnectionTrait>(
        db: &'a C,
        entry_id: i32,
    ) -> Result<Vec<String>, DbErr> {
        entry_tags::Entity::find()
            .select_only()
            .column(tags::Column::Name)
            .join(JoinType::Join, entry_tags::Relation::Tags.def())
            .filter(entry_tags::Column::EntryId.eq(entry_id))
            .into_values::<_, QueryAs>()
            .all(db)
            .await
    }

    pub async fn attach_tags_to_entry<'a, C: ConnectionTrait>(
        db: &'a C,
        entry_id: i32,
        tags: Vec<String>,
    ) -> Result<(), DbErr> {
        let condition = tags
            .into_iter()
            .map(|f| format!("\"tags\".\"name\" = '{}'", escape(f)))
            .collect::<Vec<_>>()
            .join(" OR ");

        let query = format!(
            "INSERT INTO \"entry_tags\" (\"entry_id\", \"tag_id\") \
            SELECT {}, \"tags\".\"id\" \
            FROM \"tags\" \
            WHERE {};",
            entry_id, condition,
        );

        db.execute(Statement::from_string(DatabaseBackend::Sqlite, query))
            .await
            .map(|_| ())
    }

    pub async fn detach_tags_from_entry<'a, C: ConnectionTrait>(
        db: &'a C,
        entry_id: i32,
        tags: Vec<String>,
    ) -> Result<(), DbErr> {
        let condition = tags
            .into_iter()
            .map(|f| format!("\"tags\".\"name\" = '{0}'", escape(f)))
            .collect::<Vec<_>>()
            .join(" OR ");

        let query = format!(
            "DELETE FROM \"entry_tags\" \
            WHERE \"entry_tags\".\"entry_id\" = {} \
                AND \"entry_tags\".\"tag_id\" IN (\
                    SELECT \"tags\".\"id\" \
                    FROM \"tags\" \
                    WHERE {}\
                );",
            entry_id, condition,
        );

        db.execute(Statement::from_string(DatabaseBackend::Sqlite, query))
            .await
            .map(|_| ())
    }
}
