use std::{thread::sleep, time::Duration};

use actions::{get_desired_action, Action};
use change_configuration_path::change_configuration_path;
use change_current_mod::change_mod;
use clear_configuration::clear_configuration;
use configs::Configs;
use exit::exit;
use get_desired_mod::get_desired_mod;

pub mod actions;
pub mod change_configuration_path;
pub mod change_current_mod;
pub mod clear_configuration;
pub mod configs;
pub mod exit;
pub mod functions;
pub mod get_desired_mod;
pub mod hotline_mod;
pub mod list_mods;
pub mod replace_default_music;
pub mod replace_mod;

fn main() {
    let configs = Configs::new();
    let action = get_desired_action();
    match action {
        Action::ChangeMod => change_mod(configs),
        Action::ChangeConfigurationPath => change_configuration_path(configs),
        Action::ClearConfiguration => clear_configuration(),
        Action::Exit => exit(),
    }

    sleep(Duration::from_secs(4));
}
