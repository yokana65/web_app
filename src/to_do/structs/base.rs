// pub mod to_do_items;

use super::super::enums::TaskStatus;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Base {
    pub title: String,
    pub status: TaskStatus,
}
