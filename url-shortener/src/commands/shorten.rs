use chrono::Utc;
use colored::*;
use rand::{Rng, distr::Alphanumeric, rng};
use url::Url;

use crate::{
    state::UrlEntry,
    store::{load_file, save_file},
};

const CODE_LENGTH: usize = 7;

pub fn handle_shorten(original_url: String) {
    // valudate the url we got
    if Url::parse(&original_url).is_err() {
        println!("{}", "✅ Invalid URl".red().bold());
        return;
    }

    let mut urls_data = load_file();

    if let Some(url) = urls_data
        .values()
        .find(|entry| entry.original_url == original_url)
    {
        println!("This Url Already Shortened: {}", url.code.green().bold());
        return;
    }

    let short_code = loop {
        let code: String = rng()
            .sample_iter(&Alphanumeric)
            .take(CODE_LENGTH)
            .map(|byte| byte as char)
            .collect();

        // if code already exists then the code is generated again
        if !urls_data.contains_key(&code) {
            break code;
        }
    };

    let entry = UrlEntry {
        original_url,
        code: short_code.clone(),
        created_at: Utc::now(),
    };

    urls_data.insert(short_code.clone(), entry);
    save_file(urls_data);

    println!("Code generated succesfully",);
    println!("{}", short_code.green())
}
