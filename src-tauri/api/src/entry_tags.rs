use std::collections::HashSet;

use log::{debug, error};
use sea_orm::{DbConn, DbErr, TransactionTrait};

use service::{EntryTagService, TagService};

#[tauri::command]
pub async fn get_entry_tags(
    state: tauri::State<'_, DbConn>,
    entry_id: i32,
) -> Result<Vec<String>, ()> {
    debug!("get_entry_tags CALL");
    let result = EntryTagService::get_entry_tags(state.inner(), entry_id)
        .await
        .map_err(|err| {
            error!("get_entry_tags ERROR {}", err.to_string());
            ()
        })?;

    debug!("get_entry_tags SUCCESS");
    Ok(result)
}

#[tauri::command]
pub async fn set_entry_tags(
    state: tauri::State<'_, DbConn>,
    entry_id: i32,
    tags: Vec<String>,
) -> Result<(), ()> {
    debug!("set_entry_tags CALL");

    state
        .transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                let entry_tags =
                    EntryTagService::get_entry_tags(txn, entry_id).await?;
                debug!("set_entry_tags entry_tags = {:?}", entry_tags);

                let entry_tags_set =
                    HashSet::<&String>::from_iter(entry_tags.iter());
                let tags_set = HashSet::<&String>::from_iter(tags.iter());

                let detach = entry_tags_set
                    .difference(&tags_set)
                    .collect::<HashSet<_>>();
                let attach = tags_set
                    .difference(&entry_tags_set)
                    .collect::<HashSet<_>>();

                let mut attach_ordered = Vec::new();
                for tag in &tags {
                    if attach.contains(&&tag) {
                        attach_ordered.push(tag);
                    }
                }

                let detach =
                    detach.iter().map(|s| (**s).clone()).collect::<Vec<_>>();
                let attach_ordered = attach_ordered
                    .iter()
                    .map(|s| (*s).clone())
                    .collect::<Vec<_>>();

                if attach_ordered.len() > 0 {
                    TagService::create_tags_if_abscent(
                        txn,
                        attach_ordered.clone(),
                    )
                    .await?;
                    EntryTagService::attach_tags_to_entry(
                        txn,
                        entry_id,
                        attach_ordered.clone(),
                    )
                    .await?;
                }

                debug!("set_entry_tags attach = {:?}", attach_ordered);

                if detach.len() > 0 {
                    EntryTagService::detach_tags_from_entry(
                        txn,
                        entry_id,
                        detach.clone(),
                    )
                    .await?;
                    TagService::delete_orphan_tags(txn).await?;
                }

                debug!("set_entry_tags detach = {:?}", detach);

                Ok(())
            })
        })
        .await
        .map_err(|err| {
            error!("set_entry_tags ERROR {}", err.to_string());
        })?;

    debug!("set_entry_tags SUCCESS");
    Ok(())
}
