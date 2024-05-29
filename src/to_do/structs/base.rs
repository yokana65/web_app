// super gives access to mod.rs and we meed the mod of the higher directory
use super::super::enums::TaskStatus;

pub struct Base {
    pub title: String,
    pub status: TaskStatus,
}
