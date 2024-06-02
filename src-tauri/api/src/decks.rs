use log::{debug, error};
use sea_orm::DbConn;

use entity;
use service::DeckService;

#[tauri::command]
pub async fn get_all_decks(
    state: tauri::State<'_, DbConn>,
) -> Result<Vec<entity::decks::Model>, ()> {
    debug!("get_all_decks CALL");
    let result = DeckService::find_all_decks(state.inner()).await;

    match result {
        Ok(result) => {
            debug!("get_all_decks SUCCESS");
            Ok(result)
        },
        Err(error) => {
            error!("get_all_decks ERROR {}", error.to_string());
            Ok(vec![])
        },
    }
}

#[tauri::command]
pub async fn create_deck(
    state: tauri::State<'_, DbConn>,
    deck_title: String,
) -> Result<entity::decks::Model, ()> {
    debug!("create_deck CALL");
    let result = DeckService::create_deck(
        state.inner(),
        entity::decks::Model {
            id: 0,
            name: deck_title,
        },
    )
    .await
    .map(|x| x.try_into().unwrap());

    match result {
        Ok(result) => {
            debug!("create_deck SUCCESS");
            Ok(result)
        },
        Err(error) => {
            error!("create_deck ERROR {}", error.to_string());
            Err(())
        },
    }
}

#[tauri::command]
pub async fn rename_deck(
    state: tauri::State<'_, DbConn>,
    deck_id: i32,
    new_deck_title: String,
) -> Result<entity::decks::Model, ()> {
    debug!("rename_deck CALL");
    let result = DeckService::update_deck_by_id(
        state.inner(),
        deck_id,
        entity::decks::Model {
            id: 0,
            name: new_deck_title,
        },
    )
    .await;

    match result {
        Ok(result) => {
            debug!("rename_deck SUCCESS");
            Ok(result)
        },
        Err(error) => {
            error!("rename_deck ERROR {}", error.to_string());
            Err(())
        },
    }
}

#[tauri::command]
pub async fn delete_deck(
    state: tauri::State<'_, DbConn>,
    deck_id: i32,
) -> Result<(), ()> {
    debug!("delete_deck CALL");
    let result = DeckService::delete_deck(state.inner(), deck_id).await;

    match result {
        Ok(_) => {
            debug!("delete_deck SUCCESS");
            Ok(())
        },
        Err(error) => {
            error!("delete_deck ERROR {}", error.to_string());
            Err(())
        },
    }
}

#[tauri::command]
pub async fn last_deck(
    state: tauri::State<'_, DbConn>,
) -> Result<Option<entity::decks::Model>, ()> {
    debug!("last_deck CALL");
    let result = DeckService::find_last_deck(state.inner()).await;

    match result {
        Ok(result) => {
            debug!("last_deck SUCCESS");
            Ok(result)
        },
        Err(error) => {
            error!("last_deck ERROR {}", error.to_string());
            Err(())
        },
    }
}
