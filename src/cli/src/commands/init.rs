use super::CommandTrait;
use crate::parser::Argument;
use async_trait::async_trait;
use requestty::{prompt_one, Question};
use shared::node_type::NodeType;
use std::collections::HashMap;
use std::str::FromStr;
use strum::IntoEnumIterator;

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
        // let infralink_directory = dirs::home_dir().unwrap().join(".infralink");

        let node_type_input = prompt_one(
            Question::select("node_type")
                .message(" What type of node would you like to setup?")
                .choices(NodeType::iter().map(|node_type| node_type.to_string()))
                .build(),
        )
        .unwrap();

        let node_type = NodeType::from_str(&node_type_input.as_list_item().unwrap().text).unwrap();

        match node_type {
            NodeType::Master => {}
            NodeType::Worker => {}
            NodeType::LoadBalancer => {}
        }

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
