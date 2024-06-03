use log::{debug, error};
use sea_orm::{prelude::Date, DbConn};

use entity::{entries, entry_field_values};
use service::{
    DeckService, EntryFieldValuesService, EntryKindFieldService, EntryService,
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
) -> Result<Vec<entries::Model>, ()> {
    debug!("filter_entries CALL");
    let deck =
        DeckService::find_last_deck(state.inner())
            .await
            .map_err(|err| {
                error!("filter_entries ERROR {}", err.to_string());
                ()
            })?;

    let fallback = if let Some(deck) = deck {
        format!("колода:\"{}\"", escape(deck.name))
    } else {
        "".to_string()
    };

    let result = EntryService::find(state.inner())
        .parse_with_fallback(query, fallback)
        .await;

    match result {
        Ok(result) => {
            debug!("filter_entries SUCCESS");
            Ok(result)
        }
        Err(error) => {
            error!("filter_entries ERROR {}", error);
            Err(())
        }
    }
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
    let fields = EntryKindFieldService::find_fields_by_entry_kind_id(
        state.inner(),
        entry_kind_id,
    )
    .await
    .map_err(|err| {
        error!("create_entry ERROR {}", err.to_string());
        ()
    })?;

    if fields.len() != values.len() {
        error!("create_entry ERROR mismatched field/value count");
        return Err(());
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
        state.inner(),
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
    .map(|x| x.try_into().unwrap())
    .map_err(|err| {
        error!("create_entry ERROR {}", err.to_string());
        ()
    })?;

    let entry_id = entry.id;
    EntryFieldValuesService::create_entry_field_values(
        state.inner(),
        entry_id,
        field_value_mapped,
    )
    .await
    .map_err(|err| {
        error!("create_entry ERROR {}", err.to_string());
        ()
    })?;

    // TODO: add tags

    debug!("create_entry SUCCESS");
    Ok(entry)
}
