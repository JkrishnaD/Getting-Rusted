use colored::Colorize;

use crate::store::load_file;

pub fn handle_list() {
    let file = load_file();

    if file.is_empty() {
        println!("{}", "No Url's in the file".red());
        println!("{}", "Let's generate some".green());
        return;
    }

    println!("{}", "List of the shortend url's:".blue().bold());

    for (code, entry) in file {
        println!(
            "{} -> {} (created at {})",
            code.yellow().bold(),
            entry.original_url.green(),
            entry
                .created_at
                .format("%Y-%m-%d %H:%M:%S")
                .to_string()
                .dimmed()
        );
    }
    return;
}
