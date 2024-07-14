use std::{
    fmt::Display,
    fs::{read_dir, DirEntry},
    iter::Iterator,
    path::Path,
};

use inquire::{error::InquireResult, InquireError, Select, Text};

pub fn get_user_input(prompt: &str) -> String {
    Text::new(&format!("{prompt}\n"))
        .prompt()
        .unwrap_or_else(|err| {
            if let InquireError::OperationInterrupted | InquireError::OperationCanceled = err {
                panic!("user exited the program.")
            } else {
                println!("We couldn't handle your input. Please try again.");
                get_user_input(prompt)
            }
        })
}

pub fn prompt_user_select<T: Display>(
    message: impl AsRef<str>,
    options: Vec<T>,
) -> InquireResult<T> {
    Select::new(message.as_ref(), options)
        .with_page_size(20)
        .with_help_message("Press ESC to go back.")
        .prompt()
}

pub fn get_dirs(path: &Path) -> std::io::Result<Vec<DirEntry>> {
    read_dir(path)
        .map(|dir| dir.filter_map(Result::ok))
        .map(Iterator::collect)
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
