use std::{collections::HashMap, fs, path::Path};

use crate::state::UrlEntry;

const FILE_NAME: &str = "urls.json";

pub fn load_file() -> HashMap<String, UrlEntry> {
    let path = Path::new(FILE_NAME);

    if !path.exists() {
        fs::File::create(path).expect("Failed to create file");
        return HashMap::new();
    };

    let content = fs::read_to_string(FILE_NAME).unwrap_or_default();
    serde_json::from_str(&content).unwrap_or_default()
}

pub fn save_file(data: HashMap<String, UrlEntry>) {
    let contents = serde_json::to_string_pretty(&data).expect("Failed to Serialize");
    fs::write(FILE_NAME, contents).expect("Failed to write file")
}
