use log::{debug, error};
use sea_orm::{DbConn, DbErr, TransactionTrait};

use entity::entry_kind_fields;
use service::{
    EntryKindDefaultFieldService, EntryKindField, EntryKindFieldService,
};

#[tauri::command]
pub async fn get_entry_kind_fields(
    state: tauri::State<'_, DbConn>,
    entry_kind_id: i32,
) -> Result<Vec<EntryKindField>, ()> {
    debug!("get_entry_kind_fields CALL");
    let result = EntryKindFieldService::find_fields_by_entry_kind_id(
        state.inner(),
        entry_kind_id,
    )
    .await;

    match result {
        Ok(result) => {
            debug!("get_entry_kind_fields SUCCESS");
            Ok(result)
        }
        Err(error) => {
            error!("get_entry_kind_fields ERROR {}", error.to_string());
            Err(())
        }
    }
}

#[tauri::command]
pub async fn update_entry_kind_fields(
    state: tauri::State<'_, DbConn>,
    entry_kind_id: i32,
    fields: Vec<EntryKindField>,
    deleted_fields: Vec<i32>,
) -> Result<(), ()> {
    debug!("update_entry_kind_fields CALL");

    let mut old_fields = Vec::new();
    let mut new_fields = Vec::new();
    let mut default_field_id = 0;

    for field in fields {
        if field.is_default {
            default_field_id = field.id;
        }

        if field.id > 0 {
            old_fields.push(entry_kind_fields::Model {
                id: field.id,
                entry_kind_id: 0,
                order: field.order,
                name: field.name,
                desc: field.desc,
                r#type: field.r#type,
                immutable: false,
            });
        } else {
            new_fields.push(entry_kind_fields::Model {
                id: 0,
                entry_kind_id: 0,
                order: field.order,
                name: field.name,
                desc: field.desc,
                r#type: field.r#type,
                immutable: false,
            });
        }
    }

    let result = state
        .transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                if deleted_fields.len() > 0 {
                    EntryKindFieldService::delete_entry_kind_fields(
                        txn,
                        entry_kind_id,
                        deleted_fields,
                    )
                    .await?;
                }

                if old_fields.len() > 0 {
                    EntryKindFieldService::update_entry_kind_fields(
                        txn,
                        entry_kind_id,
                        old_fields,
                    )
                    .await?;
                }

                if new_fields.len() > 0 {
                    EntryKindFieldService::create_entry_kind_fields(
                        txn,
                        entry_kind_id,
                        new_fields,
                    )
                    .await?;
                }

                EntryKindDefaultFieldService::set_entry_kind_default_field(
                    txn,
                    entry_kind_id,
                    default_field_id,
                )
                .await?;

                Ok(())
            })
        })
        .await;

    match result {
        Ok(result) => {
            debug!("update_entry_kind_fields SUCCESS");
            Ok(result)
        }
        Err(error) => {
            error!("update_entry_kind_fields ERROR {}", error.to_string());
            Err(())
        }
    }
}
