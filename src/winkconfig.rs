//mod helperror;

/// The WinkConfig struct represents command line options passed
/// to the wink command.
#[derive(serde::Serialize)]
pub struct WinkConfig {
    /// The name of the command without the path, such as wink or wink.exe.
    pub cmd_name: String,

    /// Verbose: true if the -v command line option is present. Generates more output.
    pub verbose: bool,

    /// DryRun: true if the -d command line option is present. Do not run the command.
    pub dry_run: bool,

    /// the command code entered by the user, such as EXP or CMD.
    pub command_code: String,

    /// Export: true if the -e command line option is present. Export JSON configuration.
    pub export: bool,

    /// PrettyPrint: true if the -p command line option is present. Pretty-print JSON exports.
    pub pretty_print: bool,

    /// all of the arguments on the command line, including cmd_name
    pub all_args: Vec<String>,

    /// unprocessed command line arguments to pass to the command that wink will invoke.
    pub cmd_args: Vec<String>,
    // empty unless there is a problem parsing command line arguments
    //    pub help_msg: String,
}

/// Implement the Display trait for WinkConfig to render the struct as JSON.
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
    /// The get_from_cmd_line_args function return a WinkConfig
    /// created from parsing the command line.
    pub fn new(args: Vec<String>) -> Result<WinkConfig, (WinkConfig, Box<dyn std::error::Error>)> {
        let mut dry_run: bool = false; // -d command line option
        let mut verbose: bool = false; // -v command line option
        let mut export: bool = false; // -e command line option
        let mut pretty_print: bool = false; // -p command line option
        let mut first_arg_index = 1; // number of processed command line arguments (first is command name, such as wink)
        let mut help_msg = String::new();

        for arg in args.iter().skip(first_arg_index) {
            if arg.to_lowercase() == "help" {
                help_msg = format!("Help requested by {0}", arg);
                break;
            }

            let prefix: char = arg.chars().next().unwrap();

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

            first_arg_index += 1; // just to offend C++ programmers
        }

        let mut command_code = String::new();

        if first_arg_index < args.len() {
            command_code = args[first_arg_index].to_owned();
            first_arg_index += 1;
        }

        let result = WinkConfig {
            cmd_name: regex::Regex::new(r".*[\\/](?P<name>[^\\/]+$)")
                .unwrap()
                .replace_all(args[0].as_str(), "$name")
                .to_string(),
            verbose,
            dry_run,
            command_code,
            export,
            pretty_print,
            cmd_args: (args[first_arg_index..]).to_vec(),
            all_args: args,
        };

        if help_msg.is_empty() {
            if !crate::wsl::is_windows_or_wsl() {
                help_msg = "Runs only under Windows and Windows Subsystem for Linux (WSL). Define WSL_DISTRO_NAME environment variable to override.".to_string();
            } else if result.pretty_print && !result.export {
                help_msg = "-p invalid without -e".to_string();
            } else if result.command_code.is_empty() && !(result.export || result.dry_run) {
                help_msg = "No command code found on command line".to_string();
            }
        }

        if help_msg.is_empty() {
            Ok(result)
        } else {
            Err((result, Box::new(crate::helperror::HelpError::new(help_msg))))
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    /// to pass, run tests like this:
    /// cargo test -v --all-features --all=targets --target-dir $linbld -- --show-output -epdv word a b c
    fn it_gets_from_command_line_args() {
        let mut args: Vec<String> = std::env::args().collect();

        if let Some(pos) = args
            .iter()
            .position(|x| *x == "--show-output" || *x == "--nocapture")
        {
            args.remove(pos);
        }

        let result = crate::winkconfig::WinkConfig::new(args);
        std::process::exit(match result {
            Ok(config) => {
                println!("it_gets_from_command_line_args: {0}", config);
                assert!(
                    config.cmd_name.starts_with("wink"), // cargo test adds a suffix
                    "{0} !starts_with({1})",
                    config,
                    "wink"
                );
                assert!(config.verbose);
                assert!(config.dry_run);
                assert!(config.export);
                assert!(config.pretty_print);
                assert_eq!(config.command_code, "word");
                0
            }
            Err((_config, e)) => {
                println!("{}", e);
                1
            }
        });
        //TODO: include pub all_args: Vec<String>,
        //TODO: include pub cmd_args: Vec<String>,
    }

    #[test]
    #[should_panic]
    /// this test renders the path to the parent of this tests module
    fn it_has_a_path() {
        panic!("it_has_a_path intentional panic to render path to parent of this tests module.")
    }
}
