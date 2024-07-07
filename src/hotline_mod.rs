use std::{
    ffi::OsStr,
    fmt,
    fs::ReadDir,
    path::{Path, PathBuf},
};

use crate::functions::capitalize;

pub const VALID_MUSIC_EXTENSION: &str = "wad";
pub const MUSIC_FOLDER_NAME: &str = "music";
pub const MODS_FOLDER_NAME: &str = "mods";

#[derive(Debug)]
pub struct HotlineMod {
    name: HotlineModName,
    music: Option<Music>,
    mods: AssociatedMods,
}

#[derive(Debug)]
pub struct Music(PathBuf);
#[derive(Debug)]
pub struct AssociatedMods(Vec<PathBuf>);

impl fmt::Display for HotlineMod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl HotlineMod {
    pub fn new(mod_path: &Path) -> Option<HotlineMod> {
        let name = get_name(mod_path)?;
        let music = get_music(mod_path);
        let mods = get_mods(mod_path);
        Some(HotlineMod { name, music, mods })
    }

    
    pub fn name(&self) -> &HotlineModName {
        &self.name
    }
    
    pub fn music(&self) -> Option<&PathBuf> {
        self.music.as_ref()
    }
    
    pub fn mods(&self) -> &[PathBuf] {
        &self.mods
    }
}

fn get_name(mod_path: &Path) -> Option<HotlineModName> {
    let directory_name = mod_path.file_name()?.to_str()?;

    Some(HotlineModName::from_directory(directory_name))
}
#[derive(Debug)]
pub struct HotlineModName {
    dir_name: String,
    formatted_name: String,
}

impl HotlineModName {
    pub fn from_directory(directory_name: &str) -> Self {
        let formatted_name = directory_name
            .split('_')
            .map(capitalize)
            .collect::<Vec<String>>()
            .join(" ");

        HotlineModName {
            dir_name: String::from(directory_name),
            formatted_name,
        }
    }

    pub fn directory_name(&self) -> &str {
        &self.dir_name
    }

    pub fn formatted_name(&self) -> &str {
        &self.formatted_name
    }
}

impl fmt::Display for HotlineModName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.formatted_name)
    }
}

fn get_music(mod_path: &Path) -> Option<PathBuf> {
    mod_path
        .join(MUSIC_FOLDER_NAME)
        .read_dir()
        .map(read_dir_to_path)
        .unwrap_or_default()
        .first()
        .filter(|path| is_valid_music_file(path))
        .map(ToOwned::to_owned)
}

fn get_mods(mod_path: &Path) -> Vec<PathBuf> {
    mod_path
        .join(MODS_FOLDER_NAME)
        .read_dir()
        .map(read_dir_to_path)
        .unwrap_or_default()
}

fn read_dir_to_path(read_dir: ReadDir) -> Vec<PathBuf> {
    read_dir
        .filter_map(Result::ok)
        .map(|dir| dir.path())
        .collect::<Vec<PathBuf>>()
}

fn is_valid_music_file(path: &Path) -> bool {
    path.extension().map_or(false, |extension| {
        OsStr::new(VALID_MUSIC_EXTENSION) == extension
    })
}
