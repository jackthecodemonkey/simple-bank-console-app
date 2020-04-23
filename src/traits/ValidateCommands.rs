use super::super::models::Commands::ValidCommands;

pub trait ValidateCommands {
    fn validate(&self, valid_commands: &ValidCommands) -> Result<(), String>;
}
