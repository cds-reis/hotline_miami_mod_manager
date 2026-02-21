use std::{
    collections::HashMap,
    fmt::Display,
    fs::{self, metadata, File},
    io::{self, Write},
    path::{Path, PathBuf},
    rc::Rc,
};

use thiserror::Error;

use crate::functions::get_user_input;

#[derive(Debug)]
pub struct PathsConfig {
    game: GamePath,
    mods: ModsPath,
    mods_group: ModsGroupPath,
}

impl Display for PathsConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PathsConfig(game_path:{},mods_path:{},mods_group_path:{})",
            self.game.0.to_string_lossy(),
            self.mods.0.to_string_lossy(),
            self.mods_group.0.to_string_lossy()
        )
    }
}

impl PathsConfig {
    pub fn build() -> Result<Self, PathsConfigError> {
        let Ok(file) = Self::get_path_configs_file() else {
            return Self::create_path_configs_file();
        };

        let mut entries = file
            .lines()
            .map(str::trim)
            .filter_map(|line| line.split_once(':'))
            .map(|(key, value)| (key.to_string(), PathBuf::from(value)))
            .collect::<HashMap<_, _>>();

        let game_path = entries
            .remove(GamePath::key())
            .map(GamePath::new)
            .ok_or(PathsConfigError::GamePathNotFound)?;

        let mods_path = entries
            .remove(ModsPath::key())
            .map(ModsPath::new)
            .ok_or(PathsConfigError::ModsPathNotFound)?;

        let mods_group_path = entries
            .remove(ModsGroupPath::key())
            .map(ModsGroupPath::new)
            .ok_or(PathsConfigError::ModsGroupPathNotFound)?;

        Ok(PathsConfig {
            game: game_path,
            mods: mods_path,
            mods_group: mods_group_path,
        })
    }

    pub fn clear(&self) -> Result<(), PathsConfigError> {
        fs::remove_file(PATH_CONFIGS_FILE_NAME).map_err(PathsConfigError::FileClearingError)
    }

    pub fn save(&self) -> Result<(), PathsConfigError> {
        let entries = [
            self.game.as_file_entry(),
            self.mods.as_file_entry(),
            self.mods_group.as_file_entry(),
        ];

        Self::flush_entries(&entries)
    }

    pub fn game_path(&self) -> &GamePath {
        &self.game
    }

    pub fn mods_path(&self) -> &ModsPath {
        &self.mods
    }

    pub fn mods_group_path(&self) -> &ModsGroupPath {
        &self.mods_group
    }

    pub fn set_game_path(&mut self, game_path: GamePath) {
        self.game = game_path;
    }

    pub fn set_mods_path(&mut self, mods_path: ModsPath) {
        self.mods = mods_path;
    }

    pub fn set_mods_group_path(&mut self, mods_group_path: ModsGroupPath) {
        self.mods_group = mods_group_path;
    }

    pub fn with_mods_path(self, mods_path: ModsPath) -> Self {
        PathsConfig {
            mods: mods_path,
            ..self
        }
    }
    pub fn with_mods_group_path(self, mods_group_path: ModsGroupPath) -> Self {
        PathsConfig {
            mods_group: mods_group_path,
            ..self
        }
    }

    fn get_path_configs_file() -> Result<String, PathsConfigError> {
        let configs = fs::read_to_string(PATH_CONFIGS_FILE_NAME)
            .map_err(PathsConfigError::FileLoadingError)?;

        if configs.lines().collect::<Vec<_>>().len() != 3 {
            return Err(PathsConfigError::InvalidFileContent);
        }

        Ok(configs)
    }

    fn create_path_configs_file() -> Result<Self, PathsConfigError> {
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

        Ok(PathsConfig {
            game: game_path,
            mods: mods_path,
            mods_group: mods_group_path,
        })
    }

    fn flush_entries(entries: &[PathFileEntry]) -> Result<(), PathsConfigError> {
        _ = File::open(PATH_CONFIGS_FILE_NAME)
            .map_err(PathsConfigError::FileLoadingError)?
            .write(Self::format_paths_for_file(entries).as_bytes())
            .map_err(PathsConfigError::FileWritingError)?;

        Ok(())
    }

    fn request_path_from_user<P: ProgramPath>() -> P {
        let path = PathBuf::from(get_user_input(P::prompt()));
        if metadata(&path).is_ok() {
            P::new(path)
        } else {
            println!(
                "Could not validate your {} path, please write it again.",
                P::name()
            );
            Self::request_path_from_user()
        }
    }

    fn format_paths_for_file(entries: &[PathFileEntry]) -> String {
        let mut buffer = String::new();
        for entry in entries {
            buffer += &entry.format();
        }
        buffer
    }
}

#[derive(Error, Debug)]
pub enum PathsConfigError {
    #[error("File containing path configuration failed to load.")]
    FileLoadingError(io::Error),
    #[error("Content found in path configuration file could not be read.")]
    InvalidFileContent,
    #[error("Error trying to write paths to file.")]
    FileWritingError(io::Error),
    #[error("Game path not found in configuration file.")]
    GamePathNotFound,
    #[error("Mods path not found in configuration file.")]
    ModsPathNotFound,
    #[error("Mods group path not found in configuration file.")]
    ModsGroupPathNotFound,
    #[error("Something went wrong when deleting the file {PATH_CONFIGS_FILE_NAME}. Error: {0}")]
    FileClearingError(io::Error),
}

pub trait ProgramPath {
    fn new(path: impl Into<Rc<Path>>) -> Self;
    fn path(&self) -> &Path;
    fn key() -> &'static str;
    fn name() -> &'static str;
    fn prompt() -> &'static str;
    fn as_file_entry(&self) -> PathFileEntry<'_> {
        PathFileEntry {
            key: Self::key(),
            path: self.path(),
        }
    }
}
#[derive(Debug)]
pub struct PathFileEntry<'a> {
    key: &'static str,
    path: &'a Path,
}

impl PathFileEntry<'_> {
    fn format(&self) -> String {
        format!("{}:{}\n", self.key, self.path.display())
    }
}

#[derive(Debug, Clone)]
pub struct GamePath(Rc<Path>);

impl ProgramPath for GamePath {
    fn new(path: impl Into<Rc<Path>>) -> Self {
        GamePath(path.into())
    }

    fn path(&self) -> &Path {
        self.0.as_ref()
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
#[derive(Debug, Clone)]
pub struct ModsPath(Rc<Path>);

impl ProgramPath for ModsPath {
    fn new(path: impl Into<Rc<Path>>) -> Self {
        ModsPath(path.into())
    }

    fn path(&self) -> &Path {
        self.0.as_ref()
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
#[derive(Debug, Clone)]
pub struct ModsGroupPath(Rc<Path>);

impl ProgramPath for ModsGroupPath {
    fn new(path: impl Into<Rc<Path>>) -> Self {
        ModsGroupPath(path.into())
    }

    fn path(&self) -> &Path {
        self.0.as_ref()
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

#[derive(Debug, Clone)]
pub enum ConfigurationPath<S: PathState> {
    Game(ConfigPath<S>),
    Mods(ConfigPath<S>),
    Group(ConfigPath<S>),
}

impl<S: PathState> ConfigurationPath<S> {
    pub fn name(&self) -> &str {
        match self {
            ConfigurationPath::Game(_) => "game's",
            ConfigurationPath::Mods(_) => "mod's",
            ConfigurationPath::Group(_) => "group mod's",
        }
    }
}

impl ConfigurationPath<WithoutPath> {
    pub fn with_path(self, path: impl Into<Rc<Path>>) -> ConfigurationPath<WithPath> {
        match self {
            ConfigurationPath::Game(config_path) => {
                ConfigurationPath::Game(config_path.with_path(path))
            }
            ConfigurationPath::Mods(config_path) => {
                ConfigurationPath::Mods(config_path.with_path(path))
            }
            ConfigurationPath::Group(config_path) => {
                ConfigurationPath::Group(config_path.with_path(path))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConfigPath<S: PathState>(S);

impl From<ConfigPath<WithPath>> for GamePath {
    fn from(val: ConfigPath<WithPath>) -> Self {
        GamePath(val.0 .0)
    }
}

impl From<ConfigPath<WithPath>> for ModsPath {
    fn from(val: ConfigPath<WithPath>) -> Self {
        ModsPath(val.0 .0)
    }
}

impl From<ConfigPath<WithPath>> for ModsGroupPath {
    fn from(val: ConfigPath<WithPath>) -> Self {
        ModsGroupPath(val.0 .0)
    }
}

impl ConfigPath<WithoutPath> {
    pub const fn new() -> Self {
        ConfigPath(WithoutPath)
    }

    pub fn with_path(self, path: impl Into<Rc<Path>>) -> ConfigPath<WithPath> {
        ConfigPath(WithPath(path.into()))
    }
}

impl Default for ConfigPath<WithoutPath> {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigPath<WithPath> {
    pub fn path(&self) -> &Path {
        &self.0 .0
    }
}

pub trait PathState {}
pub struct WithPath(Rc<Path>);
impl PathState for WithPath {}
#[derive(Clone)]
pub struct WithoutPath;
impl PathState for WithoutPath {}

impl ConfigurationPath<WithoutPath> {
    pub const VARIANTS: &'static [ConfigurationPath<WithoutPath>] = &[
        ConfigurationPath::Game(ConfigPath::new()),
        ConfigurationPath::Group(ConfigPath::new()),
        ConfigurationPath::Mods(ConfigPath::new()),
    ];
}

impl Display for ConfigurationPath<WithoutPath> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            ConfigurationPath::Game(_) => "Your game's folder.",
            ConfigurationPath::Mods(_) => "Your mods' folder.",
            ConfigurationPath::Group(_) => "The folder where you keep your mods.",
        };
        write!(f, "{message}")
    }
}

impl Display for ConfigurationPath<WithPath> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            ConfigurationPath::Game(ConfigPath(WithPath(path))) => {
                format!("Your game's folder is: {}.", path.to_string_lossy())
            }
            ConfigurationPath::Mods(ConfigPath(WithPath(path))) => {
                format!("Your mods' folder is: {}.", path.to_string_lossy())
            }
            ConfigurationPath::Group(ConfigPath(WithPath(path))) => {
                format!(
                    "The folder where you keep your mods is: {}",
                    path.to_string_lossy()
                )
            }
        };

        write!(f, "{message}")
    }
}

const PATH_CONFIGS_FILE_NAME: &str = "hm_mod_manager_path_configs.conf";
