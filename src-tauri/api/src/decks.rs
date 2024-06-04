use log::{debug, error};
use sea_orm::DbConn;

use entity::decks;
use service::DeckService;

#[tauri::command]
pub async fn get_all_decks(
    state: tauri::State<'_, DbConn>,
) -> Result<Vec<decks::Model>, ()> {
    debug!("get_all_decks CALL");
    let result =
        DeckService::find_all_decks(state.inner())
            .await
            .map_err(|err| {
                error!("get_all_decks ERROR {}", err.to_string());
                ()
            })?;

    debug!("get_all_decks SUCCESS");
    Ok(result)
}

#[tauri::command]
pub async fn create_deck(
    state: tauri::State<'_, DbConn>,
    deck_title: String,
) -> Result<decks::Model, ()> {
    debug!("create_deck CALL");
    let result = DeckService::create_deck(
        state.inner(),
        decks::Model {
            id: 0,
            name: deck_title,
        },
    )
    .await
    .map(|x| x.try_into().unwrap())
    .map_err(|err| {
        error!("get_all_decks ERROR {}", err.to_string());
        ()
    })?;

    debug!("create_deck SUCCESS");
    Ok(result)
}

#[tauri::command]
pub async fn rename_deck(
    state: tauri::State<'_, DbConn>,
    deck_id: i32,
    new_deck_title: String,
) -> Result<decks::Model, ()> {
    debug!("rename_deck CALL");
    let result = DeckService::update_deck_by_id(
        state.inner(),
        deck_id,
        decks::Model {
            id: 0,
            name: new_deck_title,
        },
    )
    .await
    .map_err(|err| {
        error!("get_all_decks ERROR {}", err.to_string());
        ()
    })?;

    debug!("rename_deck SUCCESS");
    Ok(result)
}

#[tauri::command]
pub async fn delete_deck(
    state: tauri::State<'_, DbConn>,
    deck_id: i32,
) -> Result<(), ()> {
    debug!("delete_deck CALL");
    DeckService::delete_deck(state.inner(), deck_id)
        .await
        .map_err(|err| {
            error!("get_all_decks ERROR {}", err.to_string());
            ()
        })?;

    debug!("delete_deck SUCCESS");
    Ok(())
}

#[tauri::command]
pub async fn last_deck(
    state: tauri::State<'_, DbConn>,
) -> Result<Option<decks::Model>, ()> {
    debug!("last_deck CALL");
    let result =
        DeckService::find_last_deck(state.inner())
            .await
            .map_err(|err| {
                error!("get_all_decks ERROR {}", err.to_string());
                ()
            })?;

    debug!("last_deck SUCCESS");
    Ok(result)
}
