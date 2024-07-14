#![warn(clippy::pedantic)]
#![warn(clippy::perf)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::return_self_not_must_use)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_panics_doc)]

use manager::HotlineModManager;
use run_game::run_hotline_miami_2;

pub mod actions;
pub mod change_configuration_path;
pub mod configs;
pub mod create_new_mod_folder;
pub mod exit;
pub mod functions;
pub mod hotline_mod;
pub mod manager;
pub mod replace_mod;
pub mod run_game;
pub mod select_mod;

fn main() -> anyhow::Result<()> {
    let manager = HotlineModManager::build();
    match manager {
        Ok(mut manager) => manager.run(),
        Err(err) => Err(err),
    }
}
