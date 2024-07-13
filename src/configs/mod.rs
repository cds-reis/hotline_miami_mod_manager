pub mod current_mod_config;
pub mod paths_config;

use std::{fs::metadata, path::PathBuf};

use crate::{functions::get_user_input, hotline_mod::HotlineModName};

use self::{
    current_mod_config::{CurrentMod, CurrentModError},
    paths_config::PathsConfig,
};

#[derive(Debug)]
pub struct Configs {
    paths_config: PathsConfig,
    current_mod: Option<CurrentMod>,
}

impl Configs {
    pub fn build() -> anyhow::Result<Self> {
        let paths_config = PathsConfig::build()?;
        let current_mod = CurrentMod::build()
            .inspect_err(Self::on_current_mod_error)
            .ok();
        print_mod_name(current_mod.as_ref());

        Ok(Configs {
            paths_config,
            current_mod,
        })
    }

    pub fn clear(&self) -> anyhow::Result<()> {
        self.paths_config.clear()?;

        if let Some(current_mod) = &self.current_mod {
            current_mod.clear()?;
        }

        Ok(())
    }

    pub fn paths_config(&self) -> &PathsConfig {
        &self.paths_config
    }

    pub fn mut_paths_config(&mut self) -> &mut PathsConfig {
        &mut self.paths_config
    }

    pub fn current_mod(&self) -> Option<&CurrentMod> {
        self.current_mod.as_ref()
    }

    pub fn set_paths_config(
        &mut self,
        paths_config: PathsConfig,
    ) -> Result<(), paths_config::PathsConfigError> {
        self.paths_config = paths_config;
        self.paths_config.save()
    }

    pub fn set_current_mod(&mut self, current_mod: HotlineModName) -> Result<(), CurrentModError> {
        let current_mod = CurrentMod::from_mod(current_mod);
        current_mod.save()?;
        self.current_mod = Some(current_mod);
        Ok(())
    }

    fn on_current_mod_error(err: &CurrentModError) {
        if let CurrentModError::IoError(error) = err {
            println!(
                "Something wrong happened while trying to read the current mod: {}",
                error
            );
        }
    }
}

fn print_mod_name(current_mod: Option<&CurrentMod>) {
    let mod_name = current_mod
        .map(|current_mod| current_mod.name().formatted_name())
        .map_or("Uncertain...", AsRef::as_ref);

    println!("You are currently using: {mod_name}");
}

pub fn get_path(prompt: &str, path_name: &str) -> PathBuf {
    let path = PathBuf::from(get_user_input(prompt));
    let result = metadata(&path).map(|_| path);
    result.unwrap_or_else(|_| {
        println!(
            "Could not validate your {} path, please write it again.",
            path_name
        );
        get_path(prompt, path_name)
    })
}
