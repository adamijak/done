use std::str::FromStr;
use std::sync::Arc;

use cascade::cascade;
use gtk4::gdk::Display;
use gtk4::gio::File;
use gtk4::prelude::*;
use gtk4::CssProvider;
use relm4_macros::view;

use crate::{adw};
use crate::data::app::App;
use crate::services::microsoft::data::list::fetch;
use crate::services::microsoft::data::task::{add_entry, add_list_entry, get_tasks, set_completed, task_selected};
use crate::events::{DataEvent, EventHandler, UiEvent};
use crate::models::list::List;
use crate::models::service::MainService;
use crate::models::task::Task;
use crate::models::token::TokenService;
use crate::services::microsoft::models::task::ToDoTask;
use crate::services::microsoft::models::token::GraphToken;
use crate::ui::base::BaseWidgets;

pub struct Handler {}

impl Handler {
    pub fn handle_uri(files: &[File], event_handler: EventHandler) {
        let bytes = files[0].uri();
        let uri = reqwest::Url::from_str(bytes.to_string().as_str()).unwrap();
        let pairs = uri.query_pairs().next().unwrap().1;
        event_handler
            .ui_tx
            .borrow_mut()
            .try_send(UiEvent::Uri(pairs.to_string()))
            .unwrap();
    }

    pub fn handle_events(event_handler: EventHandler, service: Arc<MainService>) {
        std::thread::spawn(move || {
            use tokio::runtime::Runtime;
            let rt = Runtime::new().expect("create tokio runtime");
            rt.block_on(async {
                let mut ui_recv = event_handler.ui_rv.lock().unwrap();
                let data_tx = event_handler.data_tx.lock().unwrap();
                while let Some(event) = ui_recv.recv().await {
                    match event {
                        UiEvent::ListSelected(index) => get_tasks(index, &data_tx).await,
                        UiEvent::Fetch => {
                            fetch(&data_tx, service.clone()).await;
                        },
                        UiEvent::TaskCompleted(list_id, task_id, completed) => {
                            set_completed(list_id, task_id, completed).await
                        }
                        UiEvent::Login => App::login(service.clone()).await,
                        UiEvent::TaskSelected(task_list_id, task_id) => {
                            task_selected(task_list_id, task_id, &data_tx).await
                        }
                        UiEvent::AddTaskEntry(entry, task_list_id) => {
                            add_entry(entry, task_list_id, &data_tx).await
                        }
                        UiEvent::Uri(code) => App::uri(code, &data_tx, service.clone()).await,
                        UiEvent::AddListEntry(text) => add_list_entry(text).await,
                    }
                }
            })
        });
    }

    pub fn build_ui(application: &adw::Application, event_handler: EventHandler) {
        view! {
            window = &adw::ApplicationWindow {
                set_application: Some(application),
                set_default_width: 600,
                set_default_height: 700,
                set_width_request: 600,
                set_height_request: 700,
            }
        }
        let provider = cascade! {
            CssProvider::new();
            ..load_from_data(include_bytes!("../ui/style.css"));
        };
        gtk4::StyleContext::add_provider_for_display(
            &Display::default().expect("Could not connect to a display."),
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        let widgets = BaseWidgets::new(&window);
        Handler::handle_ui(&widgets, event_handler);
        window.show();
    }

    fn handle_ui(widgets: &BaseWidgets, event_handler: EventHandler) {
        if GraphToken::token_exists() {
            event_handler
                .ui_tx
                .borrow_mut()
                .try_send(UiEvent::Fetch)
                .expect("Send UI event");
        }

        let login_tx = event_handler.ui_tx.clone();
        widgets.login_button.connect_clicked(move |_| {
            login_tx
                .borrow_mut()
                .try_send(UiEvent::Login)
                .expect("Failed to login.")
        });
        let new_task_tx = event_handler.ui_tx.clone();
        widgets
            .sidebar
            .new_list_entry
            .connect_activate(move |entry| {
                let buffer = entry.buffer();
                new_task_tx
                    .borrow_mut()
                    .try_send(UiEvent::AddListEntry(buffer.text()))
                    .expect("Failed to send ");
                buffer.delete_text(0, None);
            });
        let ui_tx = event_handler.ui_tx.clone();

        let closure_widgets = widgets.clone();
        let future = {
            let mut data_event_receiver = event_handler
                .data_rv
                .replace(None)
                .take()
                .expect("data_event_receiver");
            async move {
                while let Some(event) = data_event_receiver.recv().await {
                    // println!("Received data event: {:?}", event);
                    match event {
                        DataEvent::UpdateLists(lists) => {
                            List::fill_lists(&closure_widgets, &lists);
                        }
                        DataEvent::UpdateTasks(task_list_id, tasks) => {
                            // Task::fill_tasks(&closure_widgets, task_list_id, &tasks.iter().map(|t| t.into()).collect(), ui_tx.clone());
                        }
                        DataEvent::UpdateDetails(task_list_id, task) => {
                            // Task::fill_details(&closure_widgets, task_list_id, *task.into(), ui_tx.clone())
                        }
                        DataEvent::Login => {
                            closure_widgets.update_welcome();
                        }
                    }
                }
            }
        };

        let c = glib::MainContext::default();
        c.spawn_local(future);
        widgets
            .sidebar
            .list
            .connect_row_activated(move |listbox, _| {
                let index = listbox.selected_row().unwrap().index() as usize;
                event_handler
                    .ui_tx
                    .borrow_mut()
                    .try_send(UiEvent::ListSelected(index))
                    .expect("Send UI event");
            });
    }
}
