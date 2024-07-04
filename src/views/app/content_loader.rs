use std::fs;


pub fn read_file(file_path: &str) -> String {
    dbg!(file_path);
    println!("Current directory: {:?}", std::env::current_dir().unwrap());
    let data: String = fs::read_to_string(
        file_path).expect("Unable to read file");
    return data
}