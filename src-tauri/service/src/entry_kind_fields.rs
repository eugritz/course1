use ::entity::entry_kind_default_field;
use ::entity::entry_kind_fields;
use sea_orm::{entity::prelude::Expr, sea_query::CaseStatement, *};
use serde::{Deserialize, Serialize};

#[derive(FromQueryResult, Serialize, Deserialize)]
pub struct EntryKindField {
    pub id: i32,
    pub order: i32,
    pub entry_kind_id: i32,
    pub name: String,
    pub desc: Option<String>,
    pub r#type: String,
    pub is_default: bool,
}

pub struct EntryKindFieldService;

impl EntryKindFieldService {
    pub async fn find_fields_by_entry_kind_id(
        db: &DbConn,
        entry_kind_id: i32,
    ) -> Result<Vec<EntryKindField>, DbErr> {
        entry_kind_fields::Entity::find()
            .filter(entry_kind_fields::Column::EntryKindId.eq(entry_kind_id))
            .join(
                JoinType::LeftJoin,
                entry_kind_fields::Relation::EntryKindDefaultField.def(),
            )
            .order_by(entry_kind_fields::Column::Order, Order::Asc)
            .expr_as_(
                CaseStatement::new()
                    .case(
                        Expr::col((
                            entry_kind_default_field::Entity,
                            entry_kind_default_field::Column::EntryKindId,
                        ))
                        .is_null(),
                        false,
                    )
                    .finally(true),
                "is_default".to_owned(),
            )
            .into_model::<EntryKindField>()
            .all(db)
            .await
    }
}
