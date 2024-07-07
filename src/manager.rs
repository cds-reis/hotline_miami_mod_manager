use std::{fs, ops::Deref};

use crate::{
    configs::{
        paths_config::{ModsGroupPath, ProgramPath},
        Configs,
    },
    hotline_mod::HotlineMod,
};

pub struct HotlineModManager {
    default_game: Option<DefaultHotlineMod>,
    all_mods: Vec<HotlineMod>,
    configs: Configs,
}

impl HotlineModManager {
    pub fn build() -> anyhow::Result<Self> {
        let configs = Configs::build()?;
        let mut all_mods = list_mods(configs.paths_config().mods_group_path())?;
        let default_game_index = all_mods
            .iter()
            .position(|hm_mod| hm_mod.name().directory_name() == "hotline_miami_2");

        let default_game = default_game_index
            .map(|index| all_mods.remove(index))
            .map(|hm_mod| DefaultHotlineMod(hm_mod));

        Ok(HotlineModManager {
            default_game,
            all_mods,
            configs,
        })
    }
    
    pub fn configs(&self) -> &Configs {
        &self.configs
    }

    pub fn configs_mut(&mut self) -> &mut Configs {
        &mut self.configs
    }
}

pub struct DefaultHotlineMod(HotlineMod);

impl DefaultHotlineMod {
    pub fn hm_mod(&self) -> &HotlineMod {
        self.deref()
    }
}

impl Deref for DefaultHotlineMod {
    type Target = HotlineMod;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn list_mods(mods_path: &ModsGroupPath) -> anyhow::Result<Vec<HotlineMod>> {
    Ok(fs::read_dir(mods_path.path())?
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter_map(|path| HotlineMod::new(&path))
            .collect())
}
