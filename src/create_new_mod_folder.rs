use std::{fs::create_dir, io, path::PathBuf};

use crate::{
    configs::{
        paths_config::{ModsGroupPath, ProgramPath},
        Configs,
    },
    functions::{get_dirs, get_user_input},
    hotline_mod::{MODS_FOLDER_NAME, MUSIC_FOLDER_NAME},
};

const GET_NEW_MOD_NAME_PROMPT: &str =
    "What will be the mod's name? (Please use the snake case identifier)";

pub fn create_new_mod_folder(configs: &Configs) {
    let new_mod_name = get_new_mod_name(configs.paths_config().mods_group_path());
    let creation_result =
        create_all_mods_dirs(&new_mod_name, configs.paths_config().mods_group_path());
    match creation_result {
        Ok(_) => println!("Successfully created the folders!"),
        Err(err) => {
            println!("Something went wrong when creating your new folders.");
            println!("Error: {:?}", err);
        }
    }
}

fn get_new_mod_name(mods_group_path: &ModsGroupPath) -> String {
    let new_mod_name = get_user_input(GET_NEW_MOD_NAME_PROMPT);
    if is_new_mod_name_valid(&new_mod_name, mods_group_path) {
        return new_mod_name;
    }
    println!("There's already a mod called {new_mod_name} in your mods directory. Please provide another one.");
    get_new_mod_name(mods_group_path)
}

fn is_new_mod_name_valid(new_mod_name: &str, mods_group_path: &ModsGroupPath) -> bool {
    let dirs = get_dirs(mods_group_path.path()).unwrap_or_default();
    dirs.iter().all(|dir| dir.file_name() != new_mod_name)
}

fn create_all_mods_dirs(new_mod_name: &str, mods_group_path: &ModsGroupPath) -> io::Result<()> {
    create_new_mod_dir(new_mod_name, mods_group_path)?;
    create_new_mod_music_dir(new_mod_name, mods_group_path)?;
    create_new_mod_mods_dir(new_mod_name, mods_group_path)?;
    Ok(())
}

fn create_new_mod_dir(new_mod_name: &str, mods_group_path: &ModsGroupPath) -> io::Result<()> {
    create_dir(mods_group_path.path().join(new_mod_name))
}

fn create_new_mod_music_dir(new_mod_name: &str, mods_group_path: &ModsGroupPath) -> io::Result<()> {
    create_dir(format_music_folder_name(new_mod_name, mods_group_path))
}

fn create_new_mod_mods_dir(new_mod_name: &str, mods_group_path: &ModsGroupPath) -> io::Result<()> {
    create_dir(format_mods_folder_name(new_mod_name, mods_group_path))
}

fn format_music_folder_name(new_mod_name: &str, mods_group_path: &ModsGroupPath) -> PathBuf {
    mods_group_path
        .path()
        .join(new_mod_name)
        .join(MUSIC_FOLDER_NAME)
}

fn format_mods_folder_name(new_mod_name: &str, mods_group_path: &ModsGroupPath) -> PathBuf {
    mods_group_path
        .path()
        .join(new_mod_name)
        .join(MODS_FOLDER_NAME)
}
