use std::{
    fs::{read_dir, DirEntry},
    path::Path,
};

use inquire::Text;

pub fn get_user_input(prompt: &str) -> String {
    return Text::new(&format!("{prompt}\n"))
        .prompt()
        .unwrap_or_else(|_| {
            println!("We couldn't handle your input. Please try again.");
            get_user_input(prompt)
        });
}

pub fn get_dirs(path: &Path) -> Vec<DirEntry> {
    read_dir(path)
        .map(|dir| dir.filter_map(Result::ok))
        .map(|dir| dir.collect())
        .unwrap_or_default()
}

pub fn capitalize(value: &str) -> String {
    let mut result = String::with_capacity(value.len());
    if let Some(first_char) = value.chars().next() {
        result.push_str(&first_char.to_uppercase().to_string());
        result.push_str(&value[1..]);
    }

    result
}

pub fn work_in_progress() {
    println!("🏗 🏗  Work in progress 🏗 🏗");
}
