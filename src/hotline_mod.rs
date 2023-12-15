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
    pub name: String,
    pub music: Option<PathBuf>,
    pub mods: Vec<PathBuf>,
}

impl fmt::Display for HotlineMod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.formatted_name();
        write!(f, "{name}")
    }
}

impl HotlineMod {
    pub fn new(mod_path: &Path) -> Option<HotlineMod> {
        let name = get_name(mod_path)?;
        let music = get_music(mod_path);
        let mods = get_mods(mod_path);
        Some(HotlineMod { name, music, mods })
    }

    pub fn formatted_name(&self) -> String {
        self.name
            .split('_')
            .map(capitalize)
            .collect::<Vec<String>>()
            .join(" ")
    }
}

fn get_name(mod_path: &Path) -> Option<String> {
    mod_path
        .file_name()
        .and_then(OsStr::to_str)
        .map(ToOwned::to_owned)
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
