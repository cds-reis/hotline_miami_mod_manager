use std::path::PathBuf;

use inquire::Select;

use crate::configs::{get_path, paths_config::{ConfigurationPaths, PathsConfig}, Configs};

pub fn change_configuration_path(configs: Configs) {
    let config_path = get_desired_path_to_change();
    let path = change_to_path();
    match config_path {
        ConfigurationPaths::Game => on_game_path(path.clone(), configs),
        ConfigurationPaths::Mods => on_mods_path(path.clone(), configs),
        ConfigurationPaths::Group => on_group_path(path.clone(), configs),
    }
    println!("Successfully changd the path to {}", path.display());
}

fn on_game_path(path: PathBuf, config: Configs) {
    let config = Configs {
        paths_config: PathsConfig {
            game_path: path,
            mods_path: config.paths_config.mods_path,
            mods_group_path: config.paths_config.mods_group_path,
        },
        current_mod: config.current_mod,
    };

    save_configs_to_file(&config);
}
fn on_mods_path(path: PathBuf, config: Configs) {
    let config = Configs {
        paths_config: PathsConfig {
            game_path: config.paths_config.game_path,
            mods_path: path,
            mods_group_path: config.paths_config.mods_group_path,
        },
        current_mod: config.current_mod,
    };

    save_configs_to_file(&config);
}
fn on_group_path(path: PathBuf, config: Configs) {
    let config = Configs {
        paths_config: PathsConfig {
            game_path: config.paths_config.game_path,
            mods_path: config.paths_config.mods_path,
            mods_group_path: path,
        },
        current_mod: config.current_mod,
    };

    save_configs_to_file(&config);
}

fn get_desired_path_to_change() -> ConfigurationPaths {
    let variants = Vec::from(ConfigurationPaths::VARIANTS);
    Select::new("What folder's path do you want to change?", variants)
        .prompt()
        .expect("Unable to get the  desired path to change.")
}

fn change_to_path() -> PathBuf {
    get_path("The new path", "")
}
