use colored::Colorize;

use crate::store::{load_file, save_file};

pub fn handle_delete_code(code: String) {
    let mut file_data = load_file();

    if !file_data.contains_key(&code) {
        println!("This code {} doesn't exist", code.red());
        return;
    }

    if file_data.remove(&code).is_some() {
        println!("Removed the {} from the file", code.green())
    }

    save_file(file_data);
}

pub fn handle_delete_all() {
    let mut file_data = load_file();

    if file_data.is_empty() {
        println!("Nothing there to remove");
        return;
    }

    file_data.clear();
    println!("All Clear {:?}", file_data);
    save_file(file_data);
}
