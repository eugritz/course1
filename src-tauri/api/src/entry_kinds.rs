use log::{debug, error};
use sea_orm::{DbConn, DbErr, TransactionTrait};

use entity::{cards, entry_kind_fields, entry_kinds};
use service::{
    CardService, EntryKindDefaultFieldService, EntryKindFieldService,
    EntryKindService,
};

#[tauri::command]
pub async fn get_all_entry_kinds(
    state: tauri::State<'_, DbConn>,
) -> Result<Vec<entry_kinds::Model>, ()> {
    debug!("get_all_entry_kinds CALL");
    let result = EntryKindService::find_all_entry_kinds(state.inner()).await;

    match result {
        Ok(result) => {
            debug!("get_all_entry_kinds SUCCESS");
            Ok(result)
        }
        Err(error) => {
            error!("get_all_entry_kinds ERROR {}", error.to_string());
            Ok(vec![])
        }
    }
}

#[tauri::command]
pub async fn create_entry_kind(
    state: tauri::State<'_, DbConn>,
    entry_kind_id: i32,
    entry_kind_name: String,
) -> Result<entry_kinds::Model, ()> {
    debug!("create_entry_kind CALL");

    let result = state.transaction::<_, entry_kinds::Model, DbErr>(|txn| {
        Box::pin(async move {
            let entry_kind =
                EntryKindService::find_entry_kind_by_id(txn, entry_kind_id)
                    .await;

            if entry_kind.is_err() || entry_kind.unwrap().is_none() {
                return Err(DbErr::Custom("cloning entry kind not found".to_string()));
            }

            let result: entry_kinds::Model = EntryKindService::create_entry_kind(
                txn,
                entry_kinds::Model {
                    id: 0,
                    name: entry_kind_name,
                    immutable: false,
                },
            )
            .await
            .map(|x| x.try_into().unwrap())?;

            let fields = EntryKindFieldService::find_fields_by_entry_kind_id(
                txn,
                entry_kind_id,
            )
            .await?;

            let new_entry_kind_id = result.id;
            let mut new_fields = Vec::new();

            let mut default_field_order = 0;
            let last_field_order = fields.last().map(|f| f.id).unwrap_or(0);

            for field in fields {
                if field.is_default {
                    default_field_order = field.order;
                }

                new_fields.push(entry_kind_fields::Model {
                    id: 0,
                    entry_kind_id: new_entry_kind_id,
                    order: field.order,
                    name: field.name.clone(),
                    desc: field.desc.clone(),
                    r#type: field.r#type.clone(),
                    immutable: false,
                });
            }

            let last_new_field = EntryKindFieldService::create_entry_kind_fields(
                txn,
                new_entry_kind_id,
                new_fields,
            )
            .await?;

            if default_field_order != 0 && last_field_order != 0 {
                EntryKindDefaultFieldService::create_entry_kind_default_field(
                    txn,
                    new_entry_kind_id,
                    last_new_field.last_insert_id - last_field_order
                        + default_field_order,
                )
                .await?;
            }

            let cards = CardService::find_all_entry_kind_cards(
                txn,
                entry_kind_id
            )
            .await?;

            for card in cards {
                CardService::create_card(txn, cards::Model {
                    id: 0,
                    entry_kind_id: new_entry_kind_id,
                    name: card.name,
                    source_front: card.source_front,
                    source_back: card.source_back,
                    source_style: card.source_style,
                    immutable: false,
                })
                .await?;
            }

            Ok(result)
        })
    })
    .await;

    match result {
        Ok(result) => {
            debug!("create_entry_kind SUCCESS");
            Ok(result)
        }
        Err(error) => {
            debug!("create_entry_kind ERROR {}", error.to_string());
            Err(())
        }
    }
}

#[tauri::command]
pub async fn rename_entry_kind(
    state: tauri::State<'_, DbConn>,
    entry_kind_id: i32,
    new_entry_kind_name: String,
) -> Result<entry_kinds::Model, ()> {
    debug!("rename_entry_kind CALL");
    let result = EntryKindService::update_entry_kind_by_id(
        state.inner(),
        entry_kind_id,
        entry_kinds::Model {
            id: 0,
            name: new_entry_kind_name,
            immutable: false,
        },
    )
    .await;

    match result {
        Ok(result) => {
            debug!("rename_entry_kind SUCCESS");
            Ok(result)
        }
        Err(error) => {
            error!("rename_entry_kind ERROR {}", error.to_string());
            Err(())
        }
    }
}

#[tauri::command]
pub async fn delete_entry_kind(
    state: tauri::State<'_, DbConn>,
    entry_kind_id: i32,
) -> Result<(), ()> {
    debug!("delete_entry_kind CALL");
    let result =
        EntryKindService::delete_entry_kind(state.inner(), entry_kind_id).await;

    match result {
        Ok(_) => {
            debug!("delete_entry_kind SUCCESS");
            Ok(())
        }
        Err(error) => {
            error!("delete_entry_kind ERROR {}", error.to_string());
            Err(())
        }
    }
}

#[tauri::command]
pub async fn last_entry_kind(
    state: tauri::State<'_, DbConn>,
) -> Result<Option<entry_kinds::Model>, ()> {
    debug!("last_entry_kind CALL");
    let result = EntryKindService::find_last_entry_kind(state.inner()).await;

    match result {
        Ok(result) => {
            debug!("last_entry_kind SUCCESS");
            Ok(result)
        }
        Err(error) => {
            error!("last_entry_kind ERROR {}", error.to_string());
            Err(())
        }
    }
}
