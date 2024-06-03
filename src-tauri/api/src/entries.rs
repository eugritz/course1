use log::{debug, error};
use sea_orm::DbConn;

use entity::entries;
use service::{DeckService, EntryService};

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
