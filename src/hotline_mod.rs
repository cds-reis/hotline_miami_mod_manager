use std::{fmt, fs::ReadDir, path::Path, rc::Rc};

use crate::functions::capitalize;

pub const VALID_MUSIC_EXTENSION: &str = "wad";
pub const MUSIC_FOLDER_NAME: &str = "music";
pub const MODS_FOLDER_NAME: &str = "mods";

#[derive(Debug, Clone)]
pub struct HotlineMod {
    name: HotlineModName,
    music: Option<Music>,
    mods: AssociatedMods,
}

#[derive(Debug, Clone)]
pub struct Music(Rc<Path>);

impl Music {
    pub fn path(&self) -> &Path {
        &self.0
    }
}

impl AsRef<Path> for Music {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct AssociatedMods(Rc<[Rc<Path>]>);

impl AssociatedMods {
    pub fn mods(&self) -> &[Rc<Path>] {
        &self.0
    }
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

    pub fn from_name(name: HotlineModName) -> HotlineMod {
        let mod_path = name.directory_name();
        HotlineMod {
            music: get_music(mod_path),
            mods: get_mods(mod_path),
            name,
        }
    }

    pub fn name(&self) -> &HotlineModName {
        &self.name
    }

    pub fn music(&self) -> Option<&Music> {
        self.music.as_ref()
    }

    pub fn mods(&self) -> &AssociatedMods {
        &self.mods
    }
}

fn get_name(mod_path: &Path) -> Option<HotlineModName> {
    let directory_name = mod_path.file_name()?.to_str()?;

    Some(HotlineModName::from_directory(directory_name))
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HotlineModName {
    dir_name: Rc<Path>,
    formatted_name: Rc<str>,
}

impl HotlineModName {
    pub fn from_directory(directory_name: impl AsRef<Path>) -> Self {
        let directory_name = directory_name.as_ref();
        let formatted_name = directory_name
            .as_os_str()
            .to_string_lossy()
            .split('_')
            .map(capitalize)
            .collect::<Vec<String>>()
            .join(" ");

        HotlineModName {
            dir_name: Rc::from(directory_name),
            formatted_name: formatted_name.into(),
        }
    }

    pub fn directory_name(&self) -> &Path {
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

fn get_music(mod_path: &Path) -> Option<Music> {
    mod_path
        .join(MUSIC_FOLDER_NAME)
        .read_dir()
        .map_or_else(|_| Rc::new([]), read_dir_to_path)
        .first()
        .filter(|path| is_valid_music_file(path))
        .map(ToOwned::to_owned)
        .map(Music)
}

fn get_mods(mod_path: &Path) -> AssociatedMods {
    let associated_mods = mod_path
        .join(MODS_FOLDER_NAME)
        .read_dir()
        .map_or_else(|_| Rc::new([]), read_dir_to_path);

    AssociatedMods(associated_mods)
}

fn read_dir_to_path(read_dir: ReadDir) -> Rc<[Rc<Path>]> {
    read_dir
        .filter_map(Result::ok)
        .map(|dir| Rc::from(dir.path()))
        .collect::<Rc<_>>()
}

fn is_valid_music_file(path: &Path) -> bool {
    path.extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case(VALID_MUSIC_EXTENSION))
}
