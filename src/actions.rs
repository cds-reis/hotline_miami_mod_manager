use std::fmt::Display;

use inquire::Select;

#[derive(Debug, Clone, Copy)]
pub enum Action {
    ChangeMod,
    ChangeConfigurationPath,
    ClearConfiguration,
    Exit,
}

impl Action {
    const VARIANTS: &'static [Action] = &[
        Action::ChangeMod,
        Action::ChangeConfigurationPath,
        Action::ClearConfiguration,
        Action::Exit,
    ];
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::ChangeMod => write!(f, "Change the current mod."),
            Action::ChangeConfigurationPath => write!(f, "Change one of your paths."),
            Action::ClearConfiguration => write!(f, "Clear your configuration."),
            Action::Exit => write!(f, "Exit."),
        }
    }
}

pub fn get_desired_action() -> Action {
    Select::new("What do you want to do?", Vec::from(Action::VARIANTS))
        .prompt()
        .expect("Error trying to read you input.")
}
