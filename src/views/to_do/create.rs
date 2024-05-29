use actix_web::HttpRequest;
use serde_json::Map;
use serde_json::value::Value;

use crate::to_do::{to_do_factory, enums::TaskStatus};
use crate::processes::process_input;
use crate::state::read_file;

pub async fn create(req: HttpRequest) -> String {
    // TODO: Code panics if there is no state.json
    let state: Map<String, Value> = read_file("./state.json");

    let title = req.match_info().get("title").unwrap().to_string();
    let item = to_do_factory(&title.as_str(), TaskStatus::PENDING);

    process_input(item, "create".to_string(), &state);

    return format!("{} created", title);
}