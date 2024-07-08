pub mod current_mod_config;
pub mod paths_config;

use std::{
    collections::HashMap,
    fs::{self, metadata},
    io,
    path::PathBuf,
};

use crate::{functions::get_user_input, hotline_mod::HotlineModName};

use self::{
    current_mod_config::{CurrentMod, CurrentModError},
    paths_config::PathsConfig,
};

#[derive(Debug)]
pub struct Configs {
    paths_config: PathsConfig,
    current_mod: Option<CurrentMod>,
}

impl Configs {
    pub fn build() -> anyhow::Result<Self> {
        let paths_config = PathsConfig::build()?;
        let current_mod = CurrentMod::build()
            .inspect_err(Self::on_current_mod_error)
            .ok();
        print_mod_name(current_mod.as_ref());

        Ok(Configs {
            paths_config,
            current_mod,
        })
    }

    pub fn clear(&self) -> anyhow::Result<()> {
        self.paths_config.clear()?;

        if let Some(current_mod) = &self.current_mod {
            current_mod.clear()?;
        }

        Ok(())
    }

    pub fn paths_config(&self) -> &PathsConfig {
        &self.paths_config
    }

    pub fn current_mod(&self) -> Option<&CurrentMod> {
        self.current_mod.as_ref()
    }

    pub fn set_paths_config(
        &mut self,
        paths_config: PathsConfig,
    ) -> Result<(), paths_config::PathsConfigError> {
        self.paths_config = paths_config;
        self.paths_config.save()
    }

    pub fn set_current_mod(&mut self, current_mod: HotlineModName) -> Result<(), CurrentModError> {
        let current_mod = CurrentMod::from_mod(current_mod);
        current_mod.save()?;
        self.current_mod = Some(current_mod);
        Ok(())
    }

    fn on_current_mod_error(err: &CurrentModError) {
        if let CurrentModError::IoError(error) = err {
            println!(
                "Something wrong happened while trying to read the current mod: {}",
                error
            );
        }
    }
}

fn print_mod_name(current_mod: Option<&CurrentMod>) {
    let mod_name = current_mod
        .map(|current_mod| current_mod.name().formatted_name())
        .map_or("Uncertain...", AsRef::as_ref);

    println!("You are currently using: {mod_name}");
}

fn format_paths_for_file(paths: &[(&str, &PathBuf)]) -> String {
    paths
        .iter()
        .map(|path| (path.0, path.1.display().to_string()))
        .fold(String::new(), |mut acc, path| {
            acc += &format!("{}: {}\n", path.0, path.1);

            acc
        })
}

fn format_current_mod_for_file(current_mod: &CurrentMod) -> String {
    let dir_name = current_mod.name().directory_name();
    format!("{}: {}\n", CURRENT_MOD_KEY, dir_name.to_string_lossy())
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
        .filter_map(|s| s.split_once(':'))
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
