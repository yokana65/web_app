use serde_json::value::Value;
use serde_json::Map;

pub trait Get {
    fn get(&self, title: &String, state: &Map<String, Value>) {
        let item = state.get(title);
        match item {
            Some(value) => {
                println!("\nItem: {}", title);
                println!("\nStatus: {}", value);
            }
            None => {
                println!("\nItem: {} not found", title);
            }
        }
    }
}
