use std::process::{exit, Command};

pub fn run_hotline_miami_2() -> ! {
    let mut command = Command::new("cmd");
    let program = command.args(["/C", "start", "", HOTLINE_MIAMI_2_STEAM_URL]);
    let output = program.output();
    match output {
        Ok(_) => exit(0),
        Err(err) => {
            eprintln!("Something went wrong while opening the game.");
            eprintln!("Error: {:?}", err);
            exit(123)
        },
    }
}

const HOTLINE_MIAMI_2_STEAM_URL: &str = r"steam://rungameid/274170";
