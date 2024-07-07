use std::{
    collections::HashMap, fmt::Display, fs::{metadata, File}, io::{self, Read, Write}, path::{Path, PathBuf}
};

use thiserror::Error;

use crate::functions::get_user_input;

#[derive(Debug)]
pub struct PathsConfig {
    game_path: GamePath,
    mods_path: ModsPath,
    mods_group_path: ModsGroupPath,
}

impl PathsConfig {
    pub fn build() -> Result<PathsConfig, PathsConfigError> {
        let mut file =
            Self::get_path_configs_file().or_else(|_| Self::create_path_configs_file())?;
        let mut content = String::new();

        _ = file
            .read_to_string(&mut content)
            .map_err(PathsConfigError::InvalidFileContent)?;

        let mut entries = content
            .lines()
            .map(|line| line.trim())
            .filter_map(|line| line.split_once(':'))
            .map(|(key, value)| (key.to_string(), PathBuf::from(value)))
            .collect::<HashMap<_, _>>();

        let game_path = entries
            .remove(GamePath::key())
            .map(GamePath::new)
            .ok_or_else(|| PathsConfigError::GamePathNotFound)?;

        let mods_path = entries
            .remove(ModsPath::key())
            .map(ModsPath::new)
            .ok_or_else(|| PathsConfigError::ModsPathNotFound)?;

        let mods_group_path = entries
            .remove(ModsGroupPath::key())
            .map(ModsGroupPath::new)
            .ok_or_else(|| PathsConfigError::ModsGroupPathNotFound)?;

        Ok(PathsConfig {
            game_path,
            mods_path,
            mods_group_path,
        })
    }

    pub fn save(&self) -> Result<(), PathsConfigError> {
        let entries = [
            self.game_path.as_file_entry(),
            self.mods_path.as_file_entry(),
            self.mods_group_path.as_file_entry(),
        ];

        Self::flush_entries(&entries)
    }

    pub fn game_path(&self) -> &GamePath {
        &self.game_path
    }

    pub fn mods_path(&self) -> &ModsPath {
        &self.mods_path
    }

    pub fn mods_group_path(&self) -> &ModsGroupPath {
        &self.mods_group_path
    }

    pub fn with_game_path(self, game_path: GamePath) -> Self {
        PathsConfig { game_path, ..self }
    }

    pub fn with_mods_path(self, mods_path: ModsPath) -> Self {
        PathsConfig { mods_path, ..self }
    }
    pub fn with_mods_group_path(self, mods_group_path: ModsGroupPath) -> Self {
        PathsConfig {
            mods_group_path,
            ..self
        }
    }

    fn get_path_configs_file() -> Result<File, PathsConfigError> {
        File::open(PATH_CONFIGS_FILE_NAME).map_err(PathsConfigError::FileLoadingError)
    }

    fn create_path_configs_file() -> Result<File, PathsConfigError> {
        let mut file =
            File::create(PATH_CONFIGS_FILE_NAME).map_err(PathsConfigError::FileLoadingError)?;
        let game_path = Self::request_path_from_user::<GamePath>();
        let mods_path = Self::request_path_from_user::<ModsPath>();
        let mods_group_path = Self::request_path_from_user::<ModsGroupPath>();
        let entries = [
            game_path.as_file_entry(),
            mods_path.as_file_entry(),
            mods_group_path.as_file_entry(),
        ];

        _ = file
            .write(Self::format_paths_for_file(&entries).as_bytes())
            .map_err(PathsConfigError::FileWritingError)?;

        Ok(file)
    }

    fn flush_entries(entries: &[PathFileEntry]) -> Result<(), PathsConfigError> {
        _ = File::open(PATH_CONFIGS_FILE_NAME)
            .map_err(PathsConfigError::FileLoadingError)?
            .write(Self::format_paths_for_file(&entries).as_bytes())
            .map_err(PathsConfigError::FileWritingError)?;

        Ok(())
    }

    fn request_path_from_user<P: ProgramPath>() -> P {
        let path = PathBuf::from(get_user_input(P::prompt()));
        match metadata(&path) {
            Ok(_) => P::new(path),
            Err(_) => {
                println!(
                    "Could not validate your {} path, please write it again.",
                    P::name()
                );
                Self::request_path_from_user()
            }
        }
    }

    fn format_paths_for_file(entries: &[PathFileEntry]) -> String {
        let mut buffer = String::new();
        for entry in entries {
            buffer += &entry.format()
        }
        buffer
    }
}

#[derive(Error, Debug)]
pub enum PathsConfigError {
    #[error("File containing path configuration failed to load.")]
    FileLoadingError(io::Error),
    #[error("Content found in path configuration file could not be read.")]
    InvalidFileContent(io::Error),
    #[error("Error trying to write paths to file.")]
    FileWritingError(io::Error),
    #[error("Game path not found in configuration file.")]
    GamePathNotFound,
    #[error("Mods path not found in configuration file.")]
    ModsPathNotFound,
    #[error("Mods group path not found in configuration file.")]
    ModsGroupPathNotFound,
}

pub trait ProgramPath {
    fn new(path: PathBuf) -> Self;
    fn path(&self) -> &Path;
    fn key() -> &'static str;
    fn name() -> &'static str;
    fn prompt() -> &'static str;
    fn as_file_entry(&self) -> PathFileEntry {
        PathFileEntry {
            key: Self::key(),
            path: self.path(),
        }
    }
}
#[derive(Debug)]
struct PathFileEntry<'a> {
    key: &'static str,
    path: &'a Path,
}

impl<'a> PathFileEntry<'a> {
    fn format(&self) -> String {
        format!("{}: {}\n", self.key, self.path.display())
    }
}

#[derive(Debug)]
pub struct GamePath(PathBuf);

impl ProgramPath for GamePath {
    fn new(path: PathBuf) -> Self {
        GamePath(path)
    }

    fn path(&self) -> &Path {
        self.0.as_path()
    }

    fn key() -> &'static str {
        "game_path"
    }

    fn prompt() -> &'static str {
        "What is your HM2 path? If you are in doubt, check at C:\\SteamLibrary\\steamapps\\common"
    }

    fn name() -> &'static str {
        "game's"
    }
}
#[derive(Debug)]
pub struct ModsPath(PathBuf);

impl ProgramPath for ModsPath {
    fn new(path: PathBuf) -> Self {
        ModsPath(path)
    }

    fn path(&self) -> &Path {
        self.0.as_path()
    }

    fn key() -> &'static str {
        "mods_path"
    }

    fn prompt() -> &'static str {
        "What is your HM2 mods folder path?"
    }

    fn name() -> &'static str {
        "mods"
    }
}
#[derive(Debug)]
pub struct ModsGroupPath(PathBuf);

impl ProgramPath for ModsGroupPath {
    fn new(path: PathBuf) -> Self {
        ModsGroupPath(path)
    }

    fn path(&self) -> &Path {
        self.0.as_path()
    }

    fn key() -> &'static str {
        "mods_group_path"
    }

    fn prompt() -> &'static str {
        "What is your HM2 folder where you keep your mods?"
    }

    fn name() -> &'static str {
        "group mods"
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ConfigurationPaths {
    Game,
    Mods,
    Group,
}

impl ConfigurationPaths {
    pub const VARIANTS: &'static [ConfigurationPaths] = &[
        ConfigurationPaths::Game,
        ConfigurationPaths::Group,
        ConfigurationPaths::Mods,
    ];
}

impl Display for ConfigurationPaths {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            ConfigurationPaths::Game => "Your game's folder.",
            ConfigurationPaths::Mods => "Your mods' folder.",
            ConfigurationPaths::Group => "The folder where you keep your mods.",
        };
        write!(f, "{}", message)
    }
}

const PATH_CONFIGS_FILE_NAME: &str = "hm_mod_manager_path_configs.conf";
