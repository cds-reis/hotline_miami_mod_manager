use std::{ffi::OsStr, path::Path, fs::{DirEntry, read_dir}};

use crate::functions::capitalize;

pub fn list_mods(mods_path: &Path) -> Vec<DirEntry> {
    get_dirs(mods_path)
}

pub fn get_dirs(path: &Path) -> Vec<DirEntry> {
    read_dir(path)
        .expect("Error reading directory.")
        .filter_map(Result::ok)
        .collect::<Vec<DirEntry>>()
}


pub fn format_mod_name(name: &OsStr) -> String {
    name.to_string_lossy()
        .to_string()
        .split('_')
        .map(capitalize)
        .collect::<Vec<String>>()
        .join(" ")
}