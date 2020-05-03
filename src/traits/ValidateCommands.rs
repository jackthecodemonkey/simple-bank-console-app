use super::super::models::commands::ValidCommands;

pub trait ValidateCommands {
    fn validate(&self, valid_commands: &ValidCommands) -> Result<(), String>;
}
