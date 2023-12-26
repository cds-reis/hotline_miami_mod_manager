use std::{
    ffi::OsStr,
    fmt,
    fs::ReadDir,
    path::{Path, PathBuf},
};

use crate::functions::capitalize;

const VALID_MUSIC_EXTENSION: &str = "wad";

#[derive(Debug)]
pub struct HotlineMod {
    pub name: HotlineModName,
    pub music: Option<PathBuf>,
    pub mods: Vec<PathBuf>,
}

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
}

#[derive(Debug)]
pub struct HotlineModName(pub String);

impl HotlineModName {
    pub fn new(directory_name: &str) -> Self {
        let name = directory_name
            .split('_')
            .map(capitalize)
            .collect::<Vec<String>>()
            .join(" ");
        HotlineModName(name)
    }

    pub fn directory_name(&self) -> String {
        self.0.to_lowercase().replace(' ', "_")
    }
}

impl fmt::Display for HotlineModName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn get_name(mod_path: &Path) -> Option<HotlineModName> {
    mod_path
        .file_name()
        .and_then(OsStr::to_str)
        .map(HotlineModName::new)
}
fn get_music(mod_path: &Path) -> Option<PathBuf> {
    mod_path
        .join("music")
        .read_dir()
        .map(read_dir_to_path)
        .unwrap_or_default()
        .first()
        .filter(|path| is_valid_music_file(path))
        .map(ToOwned::to_owned)
}

fn get_mods(mod_path: &Path) -> Vec<PathBuf> {
    mod_path
        .join("mods")
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
