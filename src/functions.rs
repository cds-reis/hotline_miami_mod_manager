use std::{
    fs::{read_dir, DirEntry},
    path::Path,
    process::exit,
};

use inquire::Text;

pub fn get_user_input(prompt: &str) -> String {
    let Ok(input) = Text::new(&format!("{prompt}\n")).prompt() else {
            println!("Sorry, we couldn't handle your input.");
            exit(0);
    };
    input
}

pub fn get_dirs(path: &Path) -> Vec<DirEntry> {
    let Ok(dir) = read_dir(path) else {
        println!("Error reading the directory {}", path.display());
        exit(0);
    };
    dir.filter_map(Result::ok).collect()
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
