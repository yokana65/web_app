use serde::Serialize;
use std::vec::Vec;

use actix_web::{body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder};
use serde_json::Map;
use serde_json::Value;

use crate::state::read_file;
use crate::to_do::structs::base::Base;
use crate::to_do::ItemTypes;
use crate::to_do::{enums::TaskStatus, to_do_factory};

#[derive(Serialize)]
pub struct ToDoItems {
    pub pending_items: Vec<Base>,
    pub done_items: Vec<Base>,
    pub pending_item_count: i8,
    pub done_item_count: i8,
}

impl ToDoItems {
    pub fn new(input_items: Vec<ItemTypes>) -> Self {
        let mut pending_array_buf = Vec::new();
        let mut done_array_buf = Vec::new();

        for item in input_items {
            match item {
                ItemTypes::Pending(pending) => pending_array_buf.push(pending.super_struct),
                ItemTypes::Done(done) => done_array_buf.push(done.super_struct),
            }
        }
        let done_count = done_array_buf.len() as i8;
        let pending_count = pending_array_buf.len() as i8;

        ToDoItems {
            pending_items: pending_array_buf,
            done_items: done_array_buf,
            pending_item_count: pending_count,
            done_item_count: done_count,
        }
    }

    pub fn get_state() -> ToDoItems {
        let state: Map<String, Value> = read_file("./state.json");
        let mut array_buf = Vec::new();

        for (key, value) in state {
            let status = TaskStatus::from_string(value.as_str().unwrap().to_string());
            let item: ItemTypes = to_do_factory(&key, status);

            array_buf.push(item);
        }

        return ToDoItems::new(array_buf);
    }
}

impl Responder for ToDoItems {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
