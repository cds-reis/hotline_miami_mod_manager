use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
};

use thiserror::Error;

use crate::hotline_mod::HotlineModName;

#[derive(Debug)]
pub struct CurrentMod(HotlineModName);

impl CurrentMod {
    pub fn build() -> Result<Self, CurrentModError> {
        let mut contents = Self::get_file_contents()
            .or_else(|_| Self::create_file())
            .map_err(CurrentModError::from)?;

        let current_mod = contents
            .remove(Self::key())
            .ok_or(CurrentModError::CurrentModNotFound)?;

        let mod_name = HotlineModName::from_directory(current_mod);

        Ok(CurrentMod(mod_name))
    }

    pub fn clear(&self) -> Result<(), CurrentModError> {
        fs::remove_file(MODS_CONFIG_FILE_NAME).map_err(CurrentModError::FileClearingError)
    }

    pub fn from_mod(name: HotlineModName) -> Self {
        CurrentMod(name)
    }

    pub fn name(&self) -> &HotlineModName {
        &self.0
    }

    pub fn save(&self) -> Result<(), CurrentModError> {
        let mut file = File::options()
            .read(true)
            .write(true)
            .open(MODS_CONFIG_FILE_NAME)
            .map_err(CurrentModError::from)?;

        file.write_all(self.format_for_file().as_bytes())
            .map_err(CurrentModError::from)
    }

    fn key() -> &'static str {
        "current_mod"
    }

    fn format_for_file(&self) -> String {
        format!(
            "{}:{}\n",
            Self::key(),
            self.0.directory_name().to_string_lossy()
        )
    }

    fn get_file_contents() -> io::Result<HashMap<String, PathBuf>> {
        Ok(fs::read_to_string(MODS_CONFIG_FILE_NAME)?
            .lines()
            .map(str::trim)
            .filter_map(|line| line.split_once(':'))
            .map(|(key, value)| (key.to_string(), PathBuf::from(value)))
            .collect())
    }

    fn create_file() -> io::Result<HashMap<String, PathBuf>> {
        _ = File::create(MODS_CONFIG_FILE_NAME)?;

        Ok(HashMap::new())
    }
}

#[derive(Error, Debug)]
pub enum CurrentModError {
    #[error(
        "Something wrong happened trying to use the current mod configuration file. Error: {0}"
    )]
    IoError(#[from] io::Error),
    #[error("The current mod was not found. Maybe it's the first time using the program?")]
    CurrentModNotFound,
    #[error("Something went wrong when deleting the file {MODS_CONFIG_FILE_NAME}. Error: {0}")]
    FileClearingError(io::Error),
}

const MODS_CONFIG_FILE_NAME: &str = "hm_mod_manager_mods_configs.conf";
