use std::{ffi::OsStr, fs::read_dir, path::Path};

use crate::{functions::capitalize, hotline_mod::HotlineMod};

pub fn list_mods(mods_path: &Path) -> Vec<HotlineMod> {
    get_dirs(mods_path)
}

pub fn get_dirs(path: &Path) -> Vec<HotlineMod> {
    read_dir(path)
        .expect("Error reading directory.")
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter_map(|path| HotlineMod::new(&path))
        .collect()
}

pub fn format_mod_name(name: &OsStr) -> String {
    name.to_string_lossy()
        .to_string()
        .split('_')
        .map(capitalize)
        .collect::<Vec<String>>()
        .join(" ")
}
