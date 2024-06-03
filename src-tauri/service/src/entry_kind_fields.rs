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
    pub desc: String,
    pub r#type: String,
    pub is_default: bool,
    pub immutable: bool,
}

pub struct EntryKindFieldService;

impl EntryKindFieldService {
    pub async fn find_fields_by_entry_kind_id<'a, C: ConnectionTrait>(
        db: &'a C,
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

    pub async fn create_entry_kind_fields<'a, C: ConnectionTrait>(
        db: &'a C,
        entry_kind_id: i32,
        fields: Vec<entry_kind_fields::Model>,
    ) -> Result<InsertResult<entry_kind_fields::ActiveModel>, DbErr> {
        entry_kind_fields::Entity::insert_many(fields.iter().map(|field| {
            entry_kind_fields::ActiveModel {
                entry_kind_id: Set(entry_kind_id),
                order: Set(field.order),
                name: Set(field.name.clone()),
                desc: Set(field.desc.clone()),
                r#type: Set(field.r#type.clone()),
                ..Default::default()
            }
        }))
        .exec(db)
        .await
    }

    pub async fn update_entry_kind_fields<'a, C: ConnectionTrait>(
        db: &'a C,
        entry_kind_id: i32,
        fields: Vec<entry_kind_fields::Model>,
    ) -> Result<(), DbErr> {
        for field in fields {
            entry_kind_fields::ActiveModel {
                id: Set(field.id),
                entry_kind_id: Set(entry_kind_id),
                order: Set(field.order),
                name: Set(field.name),
                desc: Set(field.desc),
                r#type: Set(field.r#type),
                ..Default::default()
            }
            .update(db)
            .await?;
        }

        Ok(())
    }

    pub async fn delete_entry_kind_fields<'a, C: ConnectionTrait>(
        db: &'a C,
        entry_kind_id: i32,
        field_ids: Vec<i32>,
    ) -> Result<DeleteResult, DbErr> {
        let mut condition = Condition::any();
        for field_id in field_ids {
            condition = condition.add(
                Condition::all()
                    .add(entry_kind_fields::Column::Id.eq(field_id))
                    .add(
                        entry_kind_fields::Column::EntryKindId
                            .eq(entry_kind_id),
                    ),
            );
        }

        entry_kind_fields::Entity::delete_many()
            .filter(condition)
            .exec(db)
            .await
    }
}
