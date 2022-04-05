use std::sync::MutexGuard;

use tokio::sync::mpsc::Sender;

use crate::events::DataEvent;
use crate::services::microsoft::service::GraphService;
use crate::services::ToDoService;

pub async fn fetch(data_tx: &MutexGuard<'_, Sender<DataEvent>>) {
    match GraphService::get_lists_delta().await {
        Ok(lists) => data_tx
            .send(DataEvent::UpdateLists(lists))
            .await
            .expect("Failed to send UpdateLists event."),
        Err(err) => println!("{err}"),
    }
}
