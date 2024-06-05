use log::{debug, error};
use sea_orm::{prelude::Date, DbConn, DbErr, TransactionTrait};

use entity::{entries, entry_field_values, tags};
use service::{
    entry_query_builder::{Card, Entry},
    DeckService, EntryFieldValuesService, EntryKindFieldService, EntryService,
    EntryTagsService, TagService,
};

fn escape(s: String) -> String {
    let mut escaped = "".to_string();
    for ch in s.chars() {
        match ch {
            '\t' => escaped.push_str("\\t"),
            '\n' => escaped.push_str("\\n"),
            '"' => escaped.push_str("\\\""),
            '\'' => escaped.push_str("\\'"),
            '\\' => escaped.push_str("\\\\"),
            ch => escaped.push(ch),
        }
    }
    escaped
}

#[tauri::command]
pub async fn filter_entries(
    state: tauri::State<'_, DbConn>,
    query: String,
) -> Result<Vec<Entry>, ()> {
    debug!("filter_entries CALL");
    let result = 'l1: {
        let deck = DeckService::find_last_deck(state.inner()).await;
        let deck = {
            if let Err(err) = deck {
                break 'l1 Err(err);
            } else {
                deck.unwrap()
            }
        };

        let fallback = if let Some(deck) = deck {
            format!("колода:\"{}\"", escape(deck.name))
        } else {
            "".to_string()
        };

        EntryService::find(state.inner())
            .try_parse_entries(query, fallback)
            .await
    }
    .map_err(|err| {
        error!("filter_entries ERROR {}", err.to_string());
        ()
    })?;

    debug!("filter_entries SUCCESS");
    Ok(result)
}

#[tauri::command]
pub async fn filter_cards(
    state: tauri::State<'_, DbConn>,
    query: String,
) -> Result<Vec<Card>, ()> {
    debug!("filter_cards CALL");
    let result = 'l1: {
        let deck = DeckService::find_last_deck(state.inner()).await;
        let deck = {
            if let Err(err) = deck {
                break 'l1 Err(err);
            } else {
                deck.unwrap()
            }
        };

        let fallback = if let Some(deck) = deck {
            format!("колода:\"{}\"", escape(deck.name))
        } else {
            "".to_string()
        };

        EntryService::find(state.inner())
            .try_parse_cards(query, fallback)
            .await
    }
    .map_err(|err| {
        error!("filter_cards ERROR {}", err.to_string());
        ()
    })?;

    debug!("filter_cards SUCCESS");
    Ok(result)
}

#[tauri::command]
pub async fn create_entry(
    state: tauri::State<'_, DbConn>,
    entry_kind_id: i32,
    deck_id: i32,
    values: Vec<String>,
    tags: Vec<String>,
) -> Result<entries::Model, ()> {
    debug!("create_entry CALL");

    let result = state
        .transaction::<_, entries::Model, DbErr>(|txn| {
            Box::pin(async move {
                let fields =
                    EntryKindFieldService::find_fields_by_entry_kind_id(
                        txn,
                        entry_kind_id,
                    )
                    .await?;

                if fields.len() != values.len() {
                    return Err(DbErr::Custom(
                        "mismatched field/value count".to_string(),
                    ));
                }

                let field_value_mapped = fields
                    .iter()
                    .zip(values)
                    .map(|(f, v)| entry_field_values::Model {
                        id: 0,
                        entry_id: 0,
                        entry_field_id: f.id,
                        value: v,
                    })
                    .collect::<Vec<_>>();

                let entry: entries::Model = EntryService::create_entry(
                    txn,
                    entries::Model {
                        id: 0,
                        entry_kind_id,
                        deck_id,
                        color_tag: 0,
                        progress: 0.0,
                        created_at: Date::default(),
                        last_shown_at: None,
                        next_shown_at: None,
                    },
                )
                .await
                .map(|x| x.try_into().unwrap())?;

                let entry_id = entry.id;
                EntryFieldValuesService::create_entry_field_values(
                    txn,
                    entry_id,
                    field_value_mapped,
                )
                .await?;

                if tags.len() > 0 {
                    TagService::create_tags_if_abscent(txn, tags.clone())
                        .await?;
                    EntryTagsService::attach_tags_to_entry(txn, entry_id, tags)
                        .await?;
                }

                Ok(entry)
            })
        })
        .await
        .map_err(|err| {
            error!("create_entry ERROR {}", err.to_string());
            ()
        })?;

    debug!("create_entry SUCCESS");
    Ok(result)
}
