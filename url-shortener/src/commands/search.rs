use crate::store::load_file;
use colored::*;

pub fn handle_search(code: String) {
    let file_data = load_file();

    match file_data.get(&code) {
        Some(entry) => {
            println!("Url found -> {}", entry.original_url.green())
        }
        None => println!("There is no Url for this code {}", code.red()),
    }
}
