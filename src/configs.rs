use std::{
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

const CONFIGS_FILE_NAME: &str = "configs.txt";
const GAME_PATH_PROMPT: &str =
    r"What is your HM2 path? If you are in doubt, check at C:\SteamLibrary\steamapps\common";
const MODS_PATH_PROMPT: &str = r"What is your HM2 mods folder path?";
const GROUP_MODS_PATH_PROMPT: &str = r"What is your HM2 folder where you keep your mods?";

impl Configs {
    pub fn new() -> Configs {
        let (game_path, mods_path, mods_group_path) = read_contents_from_file();
        Configs {
            game_path,
            mods_path,
            mods_group_path,
        }
    }
}

impl Default for Configs {
    fn default() -> Self {
        Self::new()
    }
}

fn format_new_file_contents(game_path: &Path, mods_path: &Path, group_mods_path: &Path) -> String {
    let game_path_str = game_path.display().to_string();
    let mods_path_str = mods_path.display().to_string();
    let group_mods_path_sr = group_mods_path.display().to_string();
    format!(
        "game_path: {}\nmods_path: {}\ngroup_mods_path: {}",
        game_path_str, mods_path_str, group_mods_path_sr
    )
}

fn read_contents_from_file() -> (PathBuf, PathBuf, PathBuf) {
    let contents = fs::read_to_string(CONFIGS_FILE_NAME).unwrap_or_else(|_| create_configs_file());
    let paths = contents
        .lines()
        .map(|s| s.split_once(':'))
        .map(|s| s.expect("Invalid config data."))
        .map(|s| s.1)
        .map(str::trim)
        .map(PathBuf::from)
        .collect::<Vec<PathBuf>>();
    (paths[0].clone(), paths[1].clone(), paths[2].clone())
}

fn create_configs_file() -> String {
    let game_path = get_path(GAME_PATH_PROMPT, "game's");
    let mods_path = get_path(MODS_PATH_PROMPT, "mods'");
    let group_mods_path = get_path(GROUP_MODS_PATH_PROMPT, "group mods'");
    drop(fs::write(
        CONFIGS_FILE_NAME,
        format_new_file_contents(&game_path, &mods_path, &group_mods_path),
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
