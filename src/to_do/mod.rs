pub mod enums;
pub mod structs;
pub mod traits;

use enums::TaskStatus;
use structs::done::Done;
use structs::pending::Pending;

pub enum ItemTypes {
    Pending(Pending),
    Done(Done),
}

pub fn to_do_factory(input_title: &str, status: TaskStatus) -> ItemTypes {
    match status {
        TaskStatus::PENDING => ItemTypes::Pending(Pending::new(input_title)),
        TaskStatus::DONE => ItemTypes::Done(Done::new(input_title)),
    }
}
