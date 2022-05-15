use uuid::Uuid;

use crate::models::queryable::list::QueryableList;

#[derive(Default, Debug)]
pub struct List {
    pub id_list: String,
    pub display_name: String,
    pub is_owner: bool,
    pub count: i32,
    pub icon_name: Option<String>,
}

impl List {
    pub fn new(display_name: &str, icon_name: &str, count: i32) -> Self {
        let icon_name = if icon_name.is_empty() {
            None
        } else {
            Some(icon_name.to_string())
        };
        Self {
            id_list: Uuid::new_v4().to_string(),
            display_name: display_name.to_string(),
            is_owner: true,
            count,
            icon_name,
        }
    }
}

impl From<QueryableList> for List {
    fn from(list: QueryableList) -> Self {
        Self {
            id_list: list.id_list,
            display_name: list.display_name,
            is_owner: list.is_owner,
            count: list.count,
            icon_name: list.icon_name,
        }
    }
}

impl From<&QueryableList> for List {
    fn from(list: &QueryableList) -> Self {
        Self {
            id_list: list.id_list.clone(),
            display_name: list.display_name.clone(),
            is_owner: list.is_owner,
            count: list.count,
            icon_name: list.icon_name.clone(),
        }
    }
}