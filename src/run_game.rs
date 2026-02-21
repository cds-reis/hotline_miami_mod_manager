use std::{io, process::Command};

use thiserror::Error;

pub fn run_hotline_miami_2() -> Result<(), RunHotlineMiamiError> {
    build_command()
        .output()
        .map_err(RunHotlineMiamiError::from)
        .map(|_| ())
}

#[derive(Error, Debug)]
pub enum RunHotlineMiamiError {
    #[error("Something went wrong while opening the game. Error: {0}")]
    IoError(#[from] io::Error),
}

fn build_command() -> Command {
    if cfg!(target_os = "windows") {
        let mut command = Command::new("cmd");
        command.args(["/C", "start", "", HOTLINE_MIAMI_2_STEAM_URL]);
        command
    } else {
        let mut command = Command::new("xdg-open");
        command.arg(HOTLINE_MIAMI_2_STEAM_URL);
        command
    }
}

const HOTLINE_MIAMI_2_STEAM_URL: &str = "steam://rungameid/274170";
