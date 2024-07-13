#![warn(clippy::perf)]

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
        Err(_) => todo!(),
    }
}
