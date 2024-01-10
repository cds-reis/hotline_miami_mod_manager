use std::{
    collections::HashMap,
    fs::{self, metadata},
    io,
    path::PathBuf,
};

use crate::{functions::get_user_input, hotline_mod::HotlineModName};

#[derive(Debug)]
pub struct Configs {
    pub paths_config: PathsConfig,
    pub current_mod: Option<HotlineModName>,
}

impl Configs {
    pub fn new() -> Self {
        let paths_config = PathsConfig::new(read_path_contents_from_file());
        let mods_config = read_mods_content_from_file().ok();
        let current_mod = mods_config.and_then(|mods| {
            mods.get(CURRENT_MOD_KEY)
                .map(AsRef::as_ref)
                .map(HotlineModName::new)
        });
        print_mod_name(current_mod.as_ref());
        Configs {
            paths_config,
            current_mod,
        }
    }
}

fn print_mod_name(current_mod: Option<&HotlineModName>) {
    let mod_name = current_mod
        .map(|mod_name| &mod_name.0)
        .map_or("Uncertain...", AsRef::as_ref);

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
    fn new(configs: HashMap<String, PathBuf>) -> Self {
        PathsConfig {
            game_path: configs[GAME_PATH_KEY].clone(),
            mods_path: configs[MODS_PATH_KEY].clone(),
            mods_group_path: configs[GROUP_MODS_PATH_KEY].clone(),
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

fn read_path_contents_from_file() -> HashMap<String, PathBuf> {
    let contents =
        fs::read_to_string(PATH_CONFIGS_FILE_NAME).unwrap_or_else(|_| create_configs_file());
    contents
        .lines()
        .map(|s| s.split_once(':'))
        .map(|s| s.expect("Invalid config data."))
        .map(|s| (String::from(s.0), s.1))
        .map(|s| (s.0, s.1.trim()))
        .map(|s| (s.0, PathBuf::from(s.1)))
        .collect()
}
fn read_mods_content_from_file() -> io::Result<HashMap<String, String>> {
    let contents = fs::read_to_string(MODS_CONFIG_FILE_NAME)
        .or_else(|_| fs::write(MODS_CONFIG_FILE_NAME, String::new()).map(|_| String::new()))?;
    Ok(contents
        .lines()
        .map(|s| s.split_once(':'))
        .map(|s| s.expect("Invalid config data."))
        .map(|s| (String::from(s.0), s.1))
        .map(|s| (s.0, s.1.trim()))
        .map(|s| (s.0, String::from(s.1)))
        .collect())
}

fn create_configs_file() -> String {
    let game_path = (GAME_PATH_KEY, &get_path(GAME_PATH_PROMPT, "game's"));
    let mods_path = (MODS_PATH_KEY, &get_path(MODS_PATH_PROMPT, "mods'"));
    let group_mods_path = (
        GROUP_MODS_PATH_KEY,
        &get_path(GROUP_MODS_PATH_PROMPT, "group mods'"),
    );
    let _ = fs::write(
        PATH_CONFIGS_FILE_NAME,
        format_paths_for_file(&[game_path, mods_path, group_mods_path]),
    );
    fs::read_to_string(PATH_CONFIGS_FILE_NAME).expect("Unable to read configuration file")
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
    drop(fs::write(PATH_CONFIGS_FILE_NAME, formatted_paths));
    drop(fs::write(MODS_CONFIG_FILE_NAME, formatted_current_mod));
}

pub fn get_path(prompt: &str, path_name: &str) -> PathBuf {
    let path = PathBuf::from(get_user_input(prompt));
    let result = metadata(&path).map(|_| path);
    result.unwrap_or_else(|_| {
        println!(
            "Could not validate your {} path, please write it again.",
            path_name
        );
        get_path(prompt, path_name)
    })
}

pub const PATH_CONFIGS_FILE_NAME: &str = "hm_mod_manager_path_configs.conf";
pub const MODS_CONFIG_FILE_NAME: &str = "hm_mod_manager_mods_configs.conf";
const GAME_PATH_PROMPT: &str =
    r"What is your HM2 path? If you are in doubt, check at C:\SteamLibrary\steamapps\common";
const MODS_PATH_PROMPT: &str = r"What is your HM2 mods folder path?";
const GROUP_MODS_PATH_PROMPT: &str = r"What is your HM2 folder where you keep your mods?";
const GAME_PATH_KEY: &str = "game_path";
const MODS_PATH_KEY: &str = "mods_path";
const GROUP_MODS_PATH_KEY: &str = "group_mods_path";
const CURRENT_MOD_KEY: &str = "current_mod";
