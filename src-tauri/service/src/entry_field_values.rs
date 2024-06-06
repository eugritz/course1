use ::entity::entry_field_values;
use ::entity::entry_kind_default_field;
use ::entity::entry_kind_fields;
use sea_orm::{entity::prelude::Expr, sea_query::CaseStatement, *};
use serde::{Deserialize, Serialize};

#[derive(FromQueryResult, Serialize, Deserialize)]
pub struct EntryFieldValue {
    pub id: i32,
    pub entry_id: i32,
    pub entry_kind_id: i32,
    pub entry_field_id: i32,
    pub value: String,
    pub order: i32,
    pub name: String,
    pub desc: String,
    pub r#type: String,
    pub is_default: bool,
    pub immutable: bool,
}

pub struct EntryFieldValuesService;

impl EntryFieldValuesService {
    pub async fn find_entry_field_values<'a, C: ConnectionTrait>(
        db: &'a C,
        entry_id: i32,
    ) -> Result<Vec<entry_field_values::Model>, DbErr> {
        entry_field_values::Entity::find()
            .join(
                JoinType::Join,
                entry_field_values::Relation::EntryKindFields.def(),
            )
            .filter(entry_field_values::Column::EntryId.eq(entry_id))
            .order_by(entry_kind_fields::Column::Order, Order::Asc)
            .all(db)
            .await
    }

    pub async fn find_entry_field_values_extra<'a, C: ConnectionTrait>(
        db: &'a C,
        entry_id: i32,
    ) -> Result<Vec<EntryFieldValue>, DbErr> {
        entry_field_values::Entity::find()
            .join(
                JoinType::FullOuterJoin,
                entry_field_values::Relation::EntryKindFields.def(),
            )
            .columns(entry_kind_fields::Column::iter())
            .column_as(entry_field_values::Column::Id, "id")
            .join(
                JoinType::LeftJoin,
                entry_kind_fields::Relation::EntryKindDefaultField.def(),
            )
            .filter(entry_field_values::Column::EntryId.eq(entry_id))
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
            .into_model::<EntryFieldValue>()
            .all(db)
            .await
    }

    pub async fn create_entry_field_values<'a, C: ConnectionTrait>(
        db: &'a C,
        entry_id: i32,
        field_values: Vec<entry_field_values::Model>,
    ) -> Result<(), DbErr> {
        for fv in field_values {
            entry_field_values::ActiveModel {
                entry_id: Set(entry_id),
                entry_field_id: Set(fv.entry_field_id),
                value: Set(fv.value),
                ..Default::default()
            }
            .insert(db)
            .await?;
        }

        Ok(())
    }

    pub async fn update_entry_field_value<'a, C: ConnectionTrait>(
        db: &'a C,
        form_data: entry_field_values::Model,
    ) -> Result<(), DbErr> {
        entry_field_values::ActiveModel {
            id: Set(form_data.id),
            value: Set(form_data.value),
            ..Default::default()
        }
        .update(db)
        .await?;

        Ok(())
    }
}
