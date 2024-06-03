use ::entity::entry_field_values;
use ::entity::entry_kind_fields;
use sea_orm::*;

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
}
