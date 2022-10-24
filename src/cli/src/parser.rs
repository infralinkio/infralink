use std::{collections::HashMap, env::args};

#[derive(Debug, Clone)]
pub struct Argument {
    pub raw_args: Vec<String>,
    pub filtered_args: Vec<String>,
    pub command: String,
    pub flags: Option<HashMap<String, Option<String>>>,
}

impl Argument {
    /// Parse the flags passed in to a HashMap and insert it into the existing Argument struct.
    pub fn insert_flags(&mut self) {
        let mut flags = HashMap::new();

        // Cases we need to handle for flag parsing:
        // All below cases should work fine with a single `-` prefix as well
        // 1. --hello
        // 2. --hello=world
        // 3. --hello world
        for (arg_idx, arg) in self.raw_args.iter().enumerate() {
            // Check if the arg is a flag
            if arg.starts_with("--") || arg.starts_with("-") {
                let flag_split = arg
                    // Remove all `-` and `--` prefixes
                    .trim_start_matches("-")
                    // Split by `=` to get the flag name and value
                    .split("=")
                    .collect::<Vec<&str>>();

                let flag_name = flag_split[0].to_string();

                // Get the value of the flag if it exists, if not return None
                let mut flag_value = self.raw_args.get(arg_idx + 1).map(|s| s.to_string());

                // If the value does exist, however it is a flag then convert the value to `None`
                if flag_value.is_some() {
                    let flag_value_ref = flag_value.as_ref().unwrap();

                    if flag_value_ref.starts_with("-") || flag_value_ref.starts_with("--") {
                        flag_value = None;
                    }
                }

                // Check if the value after the = sign exists, if yes then overwrite the flag_value with the value after the = sign
                if flag_split.len() == 2 {
                    flag_value = Some(flag_split[1].to_string());
                }

                flags.insert(flag_name.to_string(), flag_value.clone());
            }
        }

        for (flag_name, flag_value) in flags.iter() {
            let flag_idx = self
                .filtered_args
                .iter()
                // Check if the s starts with a `-` and contains the flag name
                .position(|s| s.starts_with("-") && s.contains(flag_name));

            if flag_idx.is_some() {
                self.filtered_args.remove(flag_idx.unwrap());
            }

            if let Some(value) = flag_value {
                let flag_value_idx = self
                    .filtered_args
                    .iter()
                    // check if the arg is equal to the value
                    .position(|s| s == value);

                if flag_value_idx.is_some() {
                    self.filtered_args.remove(flag_value_idx.unwrap());
                }
            }
        }

        if flags.len() > 0 {
            self.flags = Some(flags);
        }
    }

    // Parse and insert the command into the existing Argument struct.
    pub fn insert_command(&mut self) {
        self.command = self.filtered_args[1].clone();
    }

    // Parse the raw arguments passed in and return a new Argument struct.
    pub async fn from_cli() -> Self {
        // Mutated during the process
        let mut args: Vec<String> = args().collect();

        // Unmutated throughout the process
        let raw_args = args.clone();

        if args.len() == 1 {
            args.push("help".to_string());
        }

        let mut result = Argument {
            raw_args,
            filtered_args: args.clone(),
            command: args[0].to_string(),
            flags: None,
        };

        result.insert_flags();

        result.insert_command();

        result
    }
}
