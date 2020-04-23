use super::super::traits::ValidateCommands::ValidateCommands;

#[derive(Debug)]
pub struct Commands {
    pub arguments: Vec<String>,
}

impl Commands {
    pub fn new(commands: Vec<String>) -> Self {
        Commands {
            arguments: commands,
        }
    }
}

#[derive(Debug)]
pub struct ValidCommands {
    pub valid_commands: Vec<String>,
}

impl ValidateCommands for Commands {
    fn validate(&self, valid_commands: &ValidCommands) -> Result<(), String> {
        let mut invalid_commands: String = String::from("");
        for argument in self.arguments.iter() {
            if !valid_commands.valid_commands.contains(argument) {
                invalid_commands.push_str("invalid command entered: ");
                invalid_commands.push_str(&argument.as_str());
                invalid_commands.push_str("\n");
            }
        }
        if invalid_commands != "" {
            return Err(invalid_commands);
        }
        Ok(())
    }
}
