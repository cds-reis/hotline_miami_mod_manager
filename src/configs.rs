use std::{
    collections::HashMap,
    fs::{self, metadata},
    path::{Path, PathBuf},
    process::exit,
    thread::sleep,
    time::Duration,
};

use crate::{functions::get_user_input, hotline_mod::HotlineModName};

#[derive(Debug)]
pub struct Configs {
    pub paths_config: PathsConfig,
    pub current_mod: Option<HotlineModName>,
}

impl Configs {
    pub fn new() -> Self {
        let configs = read_contents_from_file();
        let paths_config = PathsConfig::new(&configs);
        let current_mod = configs
            .get(CURRENT_MOD_KEY)
            .map(AsRef::as_ref)
            .map(HotlineModName::new);
        print_mod_name(current_mod.as_ref());
        Configs {
            current_mod,
            paths_config,
        }
    }
}

fn print_mod_name(current_mod: Option<&HotlineModName>) {
    let mod_name = current_mod
        .map(|mod_name| &mod_name.0)
        .map(AsRef::as_ref)
        .unwrap_or("Uncertain...");

    println!("You are currently using: {mod_name}");
}

impl Default for Configs {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct PathsConfig {
    pub game_path: PathBuf,
    pub mods_path: PathBuf,
    pub mods_group_path: PathBuf,
}

impl PathsConfig {
    fn new(configs: &HashMap<String, String>) -> Self {
        PathsConfig {
            game_path: PathBuf::from(configs[GAME_PATH_KEY].clone()),
            mods_path: PathBuf::from(configs[MODS_PATH_KEY].clone()),
            mods_group_path: PathBuf::from(configs[GROUP_MODS_PATH_KEY].clone()),
        }
    }
}

fn format_paths_for_file(paths: &[(&str, &PathBuf)]) -> String {
    paths
        .iter()
        .map(|path| (path.0, path.1.display().to_string()))
        .map(|path| format!("{}: {}\n", path.0, path.1))
        .collect()
}

fn format_current_mod_for_file(mod_name: &HotlineModName) -> String {
    let dir_name = mod_name.directory_name();
    format!("{}: {}\n", CURRENT_MOD_KEY, dir_name)
}

fn read_contents_from_file() -> HashMap<String, String> {
    let contents = fs::read_to_string(CONFIGS_FILE_NAME).unwrap_or_else(|_| create_configs_file());
    contents
        .lines()
        .map(|s| s.split_once(':'))
        .map(|s| s.expect("Invalid config data."))
        .map(|s| (String::from(s.0), s.1))
        .map(|s| (s.0, s.1.trim()))
        .map(|s| (s.0, String::from(s.1)))
        .collect()
}

fn create_configs_file() -> String {
    let game_path = (GAME_PATH_KEY, &get_path(GAME_PATH_PROMPT, "game's"));
    let mods_path = (MODS_PATH_KEY, &get_path(MODS_PATH_PROMPT, "mods'"));
    let group_mods_path = (
        GROUP_MODS_PATH_KEY,
        &get_path(GROUP_MODS_PATH_PROMPT, "group mods'"),
    );
    let _ = fs::write(
        CONFIGS_FILE_NAME,
        format_paths_for_file(&[game_path, mods_path, group_mods_path]),
    );
    fs::read_to_string(CONFIGS_FILE_NAME).expect("Unable to read configuration file")
}

pub fn save_configs_to_file(configs: &Configs) {
    let game_path: (&str, &PathBuf) = (GAME_PATH_KEY, &configs.paths_config.game_path);
    let mods_path = (MODS_PATH_KEY, &configs.paths_config.mods_path);
    let mods_group_path = (GROUP_MODS_PATH_KEY, &configs.paths_config.mods_group_path);
    let current_mod = configs.current_mod.as_ref();
    let formatted_paths = format_paths_for_file(&[game_path, mods_path, mods_group_path]);
    let formatted_current_mod = current_mod
        .map(format_current_mod_for_file)
        .unwrap_or_default();
    let file_content = formatted_paths + &formatted_current_mod;
    drop(fs::write(CONFIGS_FILE_NAME, file_content));
}

pub fn get_path(prompt: &str, path_name: &str) -> PathBuf {
    let path = PathBuf::from(get_user_input(prompt));
    panic_in_invalid_path(&path, path_name);
    path
}

fn panic_in_invalid_path(path: &Path, path_name: &str) {
    if metadata(path).is_err() {
        println!("Could not find your {} path.", path_name);
        sleep(Duration::from_secs(4));
        exit(0);
    };
}

pub const CONFIGS_FILE_NAME: &str = "hm_mod_manager_configs.conf";
const GAME_PATH_PROMPT: &str =
    r"What is your HM2 path? If you are in doubt, check at C:\SteamLibrary\steamapps\common";
const MODS_PATH_PROMPT: &str = r"What is your HM2 mods folder path?";
const GROUP_MODS_PATH_PROMPT: &str = r"What is your HM2 folder where you keep your mods?";
const GAME_PATH_KEY: &str = "game_path";
const MODS_PATH_KEY: &str = "mods_path";
const GROUP_MODS_PATH_KEY: &str = "group_mods_path";
const CURRENT_MOD_KEY: &str = "current_mod";
