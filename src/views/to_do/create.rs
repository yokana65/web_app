use actix_web::HttpRequest;
use serde_json::Map;

use crate::to_do::{to_do_factory, enums::TaskStatus};
use crate::processes::process_input;
use crate::state::read_file;

pub async fn create(req: HttpRequest) -> String {
    let state: Map<String, Value> = read_file("./state.json");

    let title = req.match_info().get("title").unwrap().to_string();
    let item = to_do_factory(&title.as_str(), TaskStatus::Pending);

    process_input(item, "create".to_string(), &state);

    format!("{} created", title);
}