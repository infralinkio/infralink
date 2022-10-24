use crate::parser::Argument;
use async_trait::async_trait;
use std::str::FromStr;
use strum::EnumString;

// All commands are defined here.
mod init;

pub struct Process {
    argument: Argument,
}

#[derive(Debug, EnumString, Clone)]
#[strum(ascii_case_insensitive)]
enum Command {
    Init,
    Master,
}

#[async_trait]
pub trait CommandTrait {
    const FLAGS: &'static [&'static str];
    type ConfigType;

    async fn execute(&self, config: Self::ConfigType) -> Result<(), String>;
    async fn validate(&self, argument: Argument) -> Result<Self::ConfigType, String>;
}

impl Process {
    pub async fn from_args(argument: Argument) -> Self {
        Self { argument }
    }

    pub async fn route(&self) {
        let command: Command = Command::from_str(&self.argument.command).unwrap();

        match command {
            Command::Init => {
                let init = init::Init {};

                let config = init.validate(self.argument.clone()).await.unwrap();

                init.execute(config).await.unwrap();
            }
            Command::Master => {}
        };
    }
}
