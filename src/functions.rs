use std::{
    fs::{create_dir, read_dir, remove_dir_all, DirEntry},
    io,
    path::Path,
};

use inquire::Text;

use crate::{configs::{save_configs_to_file, Configs}, hotline_mod::HotlineMod};

pub fn get_user_input(prompt: &str) -> String {
    return Text::new(&format!("{prompt}\n"))
        .prompt()
        .unwrap_or_else(|_| {
            println!("We couldn't handle your input. Please try again.");
            get_user_input(prompt)
        });
}

pub fn get_dirs(path: &Path) -> Result<Vec<DirEntry>, std::io::Error> {
    read_dir(path)
        .map(|dir| dir.filter_map(Result::ok))
        .map(|dir| dir.collect())
}

pub fn reset_folder(path: &Path) -> io::Result<()> {
    remove_dir_all(path)?;
    create_dir(path)?;
    Ok(())
}

pub fn capitalize(value: &str) -> String {
    let mut result = String::with_capacity(value.len());
    if let Some(first_char) = value.chars().next() {
        result.push_str(&first_char.to_uppercase().to_string());
        result.push_str(&value[1..]);
    }

    result
}

pub fn flush_configs(configs: Configs, current_mod: HotlineMod) {
    let new_config = Configs {
        current_mod: Some(current_mod.name),
        paths_config: configs.paths_config,
    };
    save_configs_to_file(&new_config);
}

pub fn work_in_progress() {
    println!("🏗 🏗  Work in progress 🏗 🏗");
}
