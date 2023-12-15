use std::{
    fs::{read_dir, DirEntry},
    path::Path,
};

use inquire::Text;

pub fn get_user_input(prompt: &str) -> String {
    // println!("{}", prompt);
    // let mut input = String::new();
    // io::stdin()
    //     .read_line(&mut input)
    //     .expect("Unable to read your input.");
    // input.trim().to_owned()
    Text::new(&format!("{prompt}\n"))
        .prompt()
        .expect("Unable to read your input")
}

pub fn get_dirs(path: &Path) -> Vec<DirEntry> {
    read_dir(path)
        .expect("Error reading directory.")
        .filter_map(Result::ok)
        .collect::<Vec<DirEntry>>()
}

pub fn capitalize(value: &str) -> String {
    let mut result = String::with_capacity(value.len());
    if let Some(first_char) = value.chars().next() {
        result.push_str(&first_char.to_uppercase().to_string());
        result.push_str(&value[1..]);
    }

    result
}
