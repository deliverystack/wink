//! This library contains the WinkConfig struct that represents
//! command line options passed to the wink command.

pub mod wsl;
use crate::wsl::inv::invocablecategory::InvocableCategory;

//pub use crate::wsl::inv::invocablecategory::InvocableCategory as InvocableCategory;

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

    /// empty unless there is a problem parsing command line arguments
    pub help_msg: String,
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
    pub fn get_from_cmd_line_args() -> WinkConfig {
        let args: Vec<String> = std::env::args().collect();
        let mut dry_run: bool = false; // -d command line option
        let mut verbose: bool = false; // -v command line option
        let mut export: bool = false; // -e command line option
        let mut pretty_print: bool = false; // -p command line option
        let mut first_arg_index = 1; // number of processed command line arguments (first is command name, such as wink)
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

            first_arg_index += 1; // just to offend C++ programmers
        }

        let mut command_code = String::new();

        if first_arg_index < args.len() {
            command_code = args[first_arg_index].to_owned();
            first_arg_index += 1;
        }

        WinkConfig {
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
            help_msg,
        }
    }
}

/// The help() function renders usage information about the wink command to stdout.
/// The msg argument is a message indicating why the command rendered usage information.
/// The args argument is the command line including the invoked command (wink) and command line arguments.
/// The categories argument contains lists of invocables used to render usage information.
pub fn help(msg: &str, config: WinkConfig, mut categories: Vec<InvocableCategory>) {
    // cmd = basename(wink.exe)
    //    let cmd = regex::Regex::new(r".*[\\/](?P<name>[^\\/]+$)").unwrap().replace_all(args[0].as_str(), "$name");
    //TODO: render invoked command line from config.
    print!(
        "
-----------------------------------------------------------------------------
{0:>12} : access  Windows and WSL features : {1}
-----------------------------------------------------------------------------
{0:>12} : invoked as : {2}
-----------------------------------------------------------------------------
{0:>12} ",
        config.cmd_name,
        msg,
        config.all_args.join(" ")
    );
    color("EXP");

    print!(
        "                explorer.exe
-----------------------------------------------------------------------------
{0:>12} ",
        config.cmd_name
    );
    color("EXP");
    print!(
        " <file.ext>     Set/open default application for extension
{0:>12} ",
        config.cmd_name
    );

    color("EXP");
    print!(
        " <shell:sendto> Invoke command code (replace <shell:sendto>)
-----------------------------------------------------------------------------

-----------------------------------------------------------------------------
{0:>12} ",
        config.cmd_name
    );
    color("CMD");
    print!(
        "                cmd.exe /c
-----------------------------------------------------------------------------
{0:>12} ",
        config.cmd_name
    );
    color("CMD");
    print!(
        " <cmd> [args]   Invoke Windows console command line
{0:>12} ",
        config.cmd_name
    );
    color("CMD");
    print!(
        " echo %PATH%    Display Windows environment variable
-----------------------------------------------------------------------------

-----------------------------------------------------------------------------
{0:>12} ",
        config.cmd_name
    );
    color("BASH");
    print!(
        "               bash.exe -c
-----------------------------------------------------------------------------
{0:>12} ",
        config.cmd_name
    );
    color("BASH");
    print!(
        " /path [args]  Invoke shell command line
{0:>12} ",
        config.cmd_name
    );
    color("BASH");
    print!(
        " echo '$USER'  Display WSL environment variable
-----------------------------------------------------------------------------

-----------------------------------------------------------------------------
{0:>12} ",
        config.cmd_name
    );
    color("CODE");
    println!(
        " [args]        See command code tables below
-----------------------------------------------------------------------------",
    );

    let mut count = 0;
    categories.sort();
    let mut terminal = term::stdout().unwrap();

    for mut category in categories {
        println!(
            "\n{0}\n-----------------------------------------------------------------------------",
            category.name
        );
        category.invocables.sort();

        for invocable in category.invocables {
            let mut desc = invocable.description;

            if desc.is_empty() {
                desc = invocable.command;
            }

            terminal.fg(term::color::BRIGHT_CYAN).unwrap();
            terminal.attr(term::Attr::Bold).unwrap();
            print!("{:>31}", invocable.command_code.to_uppercase());
            terminal.reset().unwrap();
            println!(" {}", desc);
            count += 1;
        }
    }

    println!(
        "\n{0:>12} : {1} known command codes\n",
        config.cmd_name, count
    );
    println!(
        "{0:>12} : access Windows features : {1}\n",
        config.cmd_name, msg
    );
    print!("{0:>12} [opts] <", config.cmd_name);
    color("CODE");
    println!("> [arguments]");
    println!("            -d dry (do not execute)");
    println!("            -e export (configuraiton JSON)");
    println!("            -p pretty-print (for use with -e)");
    println!("            -v verbose (print command line)\n");
    print!("{0} ", config.cmd_name);
    color("HELP");
    println!(" :                  display command usage information");
    print!("{0} ", config.cmd_name);
    color("HELP");

    if cfg!(target_os = "windows") {
        println!(" | find /i \"text\" :: identify command code matching text");
    } else {
        println!(" | grep -i \"text\" # identify command code matching text");
    }
}

/// Writes the given message to STDOUT in a color other than the default.
fn color(msg: &str) {
    let mut terminal = term::stdout().unwrap();
    terminal.fg(term::color::BRIGHT_CYAN).unwrap();
    terminal.attr(term::Attr::Bold).unwrap();
    print!("{0}", msg);
    terminal.reset().unwrap();
}


#[cfg(test)]
mod tests {
    #[test]
    /// to pass, run tests like this:
    /// cargo test -v --all-features --all=targets --target-dir $linbld -- --show-output -epdv word a b c
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
