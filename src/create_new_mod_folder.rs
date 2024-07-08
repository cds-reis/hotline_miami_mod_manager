use std::{fs::create_dir, io, ops::Not, path::Path};

use inquire::{error::InquireResult, InquireError, Text};
use thiserror::Error;

use crate::{
    configs::paths_config::{ModsGroupPath, ProgramPath},
    hotline_mod::{HotlineModName, MODS_FOLDER_NAME, MUSIC_FOLDER_NAME},
    manager::AllMods,
};

const GET_NEW_MOD_NAME_PROMPT: &str =
    "What will be the mod's name? (Please use the snake case identifier)";

pub fn create_new_mod_folder(
    all_mods: &AllMods,
    mods_group_path: &ModsGroupPath,
) -> Result<HotlineModName, CreateNewModFolderError> {
    let new_mod_name = match get_new_mod_name(all_mods) {
        Ok(new_mod) => new_mod,
        Err(InquireError::OperationCanceled) => {
            return Err(CreateNewModFolderError::UserCanceledOperation)
        }
        Err(InquireError::OperationInterrupted) => panic!("User asked to quit the program"),
        Err(err) => return Err(From::from(err)),
    };

    create_all_mods_dirs(&new_mod_name, mods_group_path)?;

    Ok(new_mod_name)
}

#[derive(Error, Debug)]
pub enum CreateNewModFolderError {
    #[error("User pressed ESC when prompted.")]
    UserCanceledOperation,
    #[error("Inquire error")]
    InquireError(#[from] InquireError),
    #[error("Error creating the folder for path {1}. Error: {0}")]
    CreatingFolderError(io::Error, Box<Path>),
}

fn get_new_mod_name(all_mods: &AllMods) -> InquireResult<HotlineModName> {
    match Text::new(&format!("{}\n", GET_NEW_MOD_NAME_PROMPT)).prompt() {
        Ok(new_name) if is_new_mod_name_valid(&new_name, all_mods) => {
            Ok(HotlineModName::from_directory(new_name))
        }
        Ok(new_mod_name) => {
            println!("There's already a mod called {new_mod_name} in your mods directory. Please provide another one.");
            get_new_mod_name(all_mods)
        }
        Err(err) => Err(err),
    }
}

fn is_new_mod_name_valid(new_mod_name: &str, all_mods: &AllMods) -> bool {
    all_mods
        .mods()
        .iter()
        .any(|hm_mod| hm_mod.name().directory_name().to_string_lossy() == new_mod_name)
        .not()
}

fn create_all_mods_dirs(
    new_mod_name: &HotlineModName,
    mods_group_path: &ModsGroupPath,
) -> Result<(), CreateNewModFolderError> {
    let new_mod_name = new_mod_name.directory_name().to_string_lossy();
    create_new_mod_dir(new_mod_name.as_ref(), mods_group_path)?;
    create_new_mod_music_dir(new_mod_name.as_ref(), mods_group_path)?;
    create_new_mod_mods_dir(new_mod_name.as_ref(), mods_group_path)?;
    Ok(())
}

fn create_new_mod_dir(
    new_mod_name: impl AsRef<str>,
    mods_group_path: &ModsGroupPath,
) -> Result<(), CreateNewModFolderError> {
    let path = mods_group_path.path().join(new_mod_name.as_ref());
    create_dir(&path)
        .map_err(|err| CreateNewModFolderError::CreatingFolderError(err, path.into_boxed_path()))
}

fn create_new_mod_music_dir(
    new_mod_name: impl AsRef<str>,
    mods_group_path: &ModsGroupPath,
) -> Result<(), CreateNewModFolderError> {
    let path = format_music_folder_name(new_mod_name, mods_group_path);
    create_dir(&path).map_err(|err| CreateNewModFolderError::CreatingFolderError(err, path))
}

fn create_new_mod_mods_dir(
    new_mod_name: impl AsRef<str>,
    mods_group_path: &ModsGroupPath,
) -> Result<(), CreateNewModFolderError> {
    let path = format_mods_folder_name(new_mod_name, mods_group_path);
    create_dir(&path).map_err(|err| CreateNewModFolderError::CreatingFolderError(err, path))
}

fn format_music_folder_name(
    new_mod_name: impl AsRef<str>,
    mods_group_path: &ModsGroupPath,
) -> Box<Path> {
    mods_group_path
        .path()
        .join(new_mod_name.as_ref())
        .join(MUSIC_FOLDER_NAME)
        .into()
}

fn format_mods_folder_name(
    new_mod_name: impl AsRef<str>,
    mods_group_path: &ModsGroupPath,
) -> Box<Path> {
    mods_group_path
        .path()
        .join(new_mod_name.as_ref())
        .join(MODS_FOLDER_NAME)
        .into_boxed_path()
}
