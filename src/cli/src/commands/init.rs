use super::CommandTrait;
use crate::parser::Argument;
use async_trait::async_trait;
use std::collections::HashMap;

// Init-specific arguments and configuration
pub struct Init {}

#[derive(Debug)]
pub struct InitConfig {
    pub flags: HashMap<String, Option<String>>,
    pub args: Vec<String>,
}

#[async_trait]
impl CommandTrait for Init {
    const FLAGS: &'static [&'static str] = &["type"];
    type ConfigType = InitConfig;

    async fn execute(&self, config: Self::ConfigType) -> Result<(), String> {
        let infralink_directory = dirs::home_dir().unwrap().join(".infralink");

        let node_type = config.args.last().unwrap();

        println!("{node_type}");

        Ok(())
    }

    async fn validate(&self, argument: Argument) -> Result<Self::ConfigType, String> {
        let mut final_flags = HashMap::new();

        if let Some(argument_flags) = argument.flags {
            argument_flags.iter().for_each(|(key, value)| {
                if Self::FLAGS.contains(&key.as_str()) {
                    final_flags.insert(key.to_string(), value.to_owned());
                } else {
                    // triggered when the wrong / invalid flags are passed in
                    // ignore this for now
                }
            });
        }

        Ok(InitConfig {
            flags: final_flags,
            args: argument.filtered_args,
        })
    }
}
