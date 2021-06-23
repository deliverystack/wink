//#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]

#[derive(serde::Serialize)]
pub struct WinkConfig {
    pub cmd_name: String,
    pub verbose: bool,
    pub dry_run: bool,
    pub command_code: String,
    pub export: bool,
    pub pretty_print: bool,
    pub all_args: Vec<String>,
    pub cmd_args: Vec<String>,
    pub help_msg: String,
}

impl std::fmt::Display for WinkConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.pretty_print {
            write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
        } else {
            write!(f, "{}", serde_json::to_string(&self).unwrap())
        }
    }
}

impl WinkConfig {
    pub fn get_from_cmd_line_args() -> WinkConfig {
        let args: Vec<String> = std::env::args().collect();
        let mut dry_run: bool = false; // -d command line option
        let mut verbose: bool = false; // -v command line option
        let mut export: bool = false; // -e command line option
        let mut pretty_print: bool = false; // -p command line option
        let mut first_arg_index = 1; // number of processed command line arguments (first is command name)
        let mut help_msg = String::new();

        for arg in args.iter().skip(first_arg_index) {
            if arg == "help" {
                help_msg = format!("Help requested by {0}", arg);
                break;
            }

            if arg == "--show-output" || arg == "--nocapture" {
                // ignore parameters to test binary from cargo test
                first_arg_index += 1;
                continue;
            }

            let prefix: char = arg.to_lowercase().chars().next().unwrap();

            // if the argument is not help and does not start with a slash or a dash, then it should be a command code
            if prefix != '/' && prefix != '-' {
                break;
            }

            for char in arg.chars() {
                match char {
                    '/' | '-' => continue,
                    'v' => verbose = true,
                    'd' => dry_run = true,
                    'p' => pretty_print = true,
                    'e' => export = true,
                    'h' | '?' => {
                        help_msg = format!("Help requested by {0}", arg);
                        break;
                    }
                    _ => {
                        help_msg = format!("Unrecognized command line option: {0}", arg);
                        break;
                    }
                }
            }

            first_arg_index += 1;
        }

        let mut command_code = String::new();

        if first_arg_index < args.len() {
            command_code = args[first_arg_index].to_owned();
            first_arg_index += 1;
        }

        WinkConfig { cmd_name: regex::Regex::new(r".*[\\/](?P<name>[^\\/]+$)").unwrap().replace_all(args[0].as_str(), "$name").to_string(), verbose, dry_run, command_code, export, pretty_print, cmd_args: (args[first_arg_index..]).to_vec(), all_args: args, help_msg }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    /// to pass, run tests like this:
    // cargo test -v --all-features --all=targets --target-dir $linbld -- --show-output -epdv word a b c
    fn it_gets_from_command_line_args() {
        let wink_config = crate::WinkConfig::get_from_cmd_line_args();
        println!("it_gets_from_command_line_args: {0}", wink_config);
        assert!(
            wink_config.cmd_name.starts_with("wink"), // cargo test adds a suffix
            "{0} !starts_with({1})",
            wink_config,
            "wink"
        );
        assert!(wink_config.verbose);
        assert!(wink_config.dry_run);
        assert!(wink_config.export);
        assert!(wink_config.pretty_print);
        assert_eq!(wink_config.command_code, "word");
        assert!(wink_config.help_msg.is_empty(), "{}", wink_config.help_msg);

        //        pub all_args: Vec<String>,
        //        pub cmd_args: Vec<String>,
    }

    #[test]
    #[should_panic]
    /// this test renders the path to the parent of this tests module
    fn it_has_a_path() {
        panic!("it_has_a_path intentional panic to render path to parent of this tests module.")
    }
}
