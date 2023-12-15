use std::{
    collections::HashMap,
    fs::{self, metadata},
    path::{Path, PathBuf},
};

use crate::functions::get_user_input;

#[derive(Debug)]
pub struct Configs {
    pub game_path: PathBuf,
    pub mods_path: PathBuf,
    pub mods_group_path: PathBuf,
}

const CONFIGS_FILE_NAME: &str = "configs.conf";
const GAME_PATH_PROMPT: &str =
    r"What is your HM2 path? If you are in doubt, check at C:\SteamLibrary\steamapps\common";
const MODS_PATH_PROMPT: &str = r"What is your HM2 mods folder path?";
const GROUP_MODS_PATH_PROMPT: &str = r"What is your HM2 folder where you keep your mods?";
const GAME_PATH_KEY: &str = "game_path";
const MODS_PATH_KEY: &str = "mods_path";
const GROUP_MODS_PATH_KEY: &str = "group_mods_path";

impl Configs {
    pub fn new() -> Self {
        let paths = read_contents_from_file();
        Configs {
            game_path: paths[GAME_PATH_KEY].clone(),
            mods_path: paths[MODS_PATH_KEY].clone(),
            mods_group_path: paths[GROUP_MODS_PATH_KEY].clone(),
        }
    }
}

impl Default for Configs {
    fn default() -> Self {
        Self::new()
    }
}

fn format_new_file_contents(paths: &[(&str, PathBuf)]) -> String {
    paths
        .iter()
        .map(|path| (path.0, path.1.display().to_string()))
        .map(|path| format!("{}: {}\n", path.0, path.1))
        .collect()
}

fn read_contents_from_file() -> HashMap<String, PathBuf> {
    let contents = fs::read_to_string(CONFIGS_FILE_NAME).unwrap_or_else(|_| create_configs_file());
    contents
        .lines()
        .map(|s| s.split_once(':'))
        .map(|s| s.expect("Invalid config data."))
        .map(|s| (String::from(s.0), s.1))
        .map(|s| (s.0, s.1.trim()))
        .map(|s| (s.0, PathBuf::from(s.1)))
        .collect::<HashMap<String, PathBuf>>()
}

fn create_configs_file() -> String {
    let game_path = (GAME_PATH_KEY, get_path(GAME_PATH_PROMPT, "game's"));
    let mods_path = (MODS_PATH_KEY, get_path(MODS_PATH_PROMPT, "mods'"));
    let group_mods_path = (
        GROUP_MODS_PATH_KEY,
        get_path(GROUP_MODS_PATH_PROMPT, "group mods'"),
    );
    drop(fs::write(
        CONFIGS_FILE_NAME,
        format_new_file_contents(&[game_path, mods_path, group_mods_path]),
    ));
    fs::read_to_string(CONFIGS_FILE_NAME).expect("Unable to read configuration file")
}

fn get_path(prompt: &str, path_name: &str) -> PathBuf {
    let path = PathBuf::from(get_user_input(prompt));
    panic_in_invalid_path(&path, path_name);
    path
}

fn panic_in_invalid_path(path: &Path, path_name: &str) {
    metadata(path).unwrap_or_else(|_| panic!("Could not find your {} path.", path_name));
}
