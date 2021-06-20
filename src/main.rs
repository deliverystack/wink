// $ wink gowindow
// wslpath: C:/temp/GoWindow.{ED7BA470-8E54-465E-825C-99712043E01C}: No such file or directory

// /mnt/c/usr/bin/xeyes - wslpath seems to be adding path incorrectly

//! wink is a command line tool that provides access to Windows and Windows Subsystem for Linux (WSL) features and programs.

//! wink provides a simple interface for invoking almost any Windows or WSL feature
//! using cmd.exe /c, using explorer.exe, using bash.exe, or by invoking the executable directly.

//! wink uses cargo, so you can use cargo build to build wink. You can also use the wince script to build wink.

//! ```//TODO: less hard-coding in wince build script```

//! Run wink with no command line parameters to get usage information.

mod wsl;

use wsl::{invocablecategory::InvocableCategory, invocablecategorylist::InvocableCategoryList, invoker::Invoker};

/// The help() function renders usage information about the wink command to stdout.
/// The msg argument is a message indicating why the command rendered usage information.
/// The args argument is the command line including the invoked command (wink) and command line arguments.
/// The categories argument contains lists of invocables used to render usage information.
fn help(msg: &str, args: Vec<String>, mut categories: Vec<InvocableCategory>) {
    // cmd = basename(wink.exe)
    let cmd = regex::Regex::new(r".*[\\/](?P<name>[^\\/]+$)").unwrap().replace_all(args[0].as_str(), "$name");

    print!(
        "
-----------------------------------------------------------------------------
{0:>12} : access  Windows and WSL features : {1}
-----------------------------------------------------------------------------

-----------------------------------------------------------------------------
{0:>12} ",
        cmd, msg
    );
    color("EXP");
    print!(
        "                explorer.exe
-----------------------------------------------------------------------------
{0:>12} ",
        cmd
    );
    color("EXP");
    print!(
        " <file.ext>     Set/open default application for extension
{0:>12} ",
        cmd
    );

    color("EXP");
    print!(
        " <shell:sendto> Invoke command code (replace <shell:sendto>)
-----------------------------------------------------------------------------

-----------------------------------------------------------------------------
{0:>12} ",
        cmd
    );
    color("CMD");
    print!(
        "                cmd.exe /c
-----------------------------------------------------------------------------
{0:>12} ",
        cmd
    );
    color("CMD");
    print!(
        " <cmd> [args]   Invoke Windows console command line
{0:>12} ",
        cmd
    );
    color("CMD");
    print!(
        " echo %PATH%    Display Windows environment variable
-----------------------------------------------------------------------------

-----------------------------------------------------------------------------
{0:>12} ",
        cmd
    );
    color("BASH");
    print!(
        "               bash.exe -c
-----------------------------------------------------------------------------
{0:>12} ",
        cmd
    );
    color("BASH");
    print!(
        " /path [args]  Invoke shell command line
{0:>12} ",
        cmd
    );
    color("BASH");
    print!(
        " echo '$USER'  Display WSL environment variable
-----------------------------------------------------------------------------

-----------------------------------------------------------------------------
{0:>12} ",
        cmd
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
        println!("\n{0}\n-----------------------------------------------------------------------------", category.name);
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

    println!("\n{0:>12} : {1} known command codes\n", cmd, count);
    println!("{0:>12} : access Windows features : {1}\n", cmd, msg);
    print!("{0:>12} [opts] <", cmd);
    color("CODE");
    println!("> [arguments]");
    println!("            -d dry (do not execute)");
    println!("            -e export (configuraiton JSON)");
    println!("            -p pretty-print (for use with -e)");
    println!("            -v verbose (print command line)\n");
    print!("{0} ", cmd);
    color("HELP");
    println!(" :                  display command usage information");
    print!("{0} ", cmd);
    color("HELP");

    if cfg!(target_os = "windows") {
        println!(" | find /i \"text\" :: identify command code matching text");
    } else {
        println!(" | grep -i \"text\" # identify command code matching text");
    }
}

/// The main() function of the program accepts command line arguments through env::args.collect()
/// rather than as parameters.
fn main() {
    // command line arguments
    let args: Vec<String> = std::env::args().collect();

    // categories contain lists of invocables that map command codes to commands
    let category_list = InvocableCategoryList::get();

    // if not running under WSL or Windows, it should not be possible to run Windows commands

    if !crate::wsl::is_windows_or_wsl() {
        help("Runs only under Windows and Windows Subsystem for Linux (WSL). Define WSL_DISTRO_NAME environment variable to override.", args, category_list.categories.to_vec());
        std::process::exit(1);
        //                return;
    }

    let mut dry_run: bool = false; // -d command line option
    let mut verbose: bool = false; // -v command line option
    let mut export: bool = false; // -e command line option
    let mut pretty: bool = false; // -p command line option
    let mut first_arg_index = 1; // number of processed command line arguments (first is command name)

    for arg in args.iter().skip(1) {
        let prefix: char = arg.to_lowercase().chars().next().unwrap();

        if arg == "help" {
            help("Help requested", args, category_list.categories.to_vec());
            std::process::exit(1);
            //            return;
        }

        // if the argument is not help and does not start with a slash or a dash, then it should be a command code
        if prefix != '/' && prefix != '-' {
            break;
        }

        for char in arg.chars() {
            // ignore slashes and dashes
            if char == '/' || char == '-' {
                continue;
            // show help for -h or -?
            } else if char == 'h' || char == '?' {
                help("Help requested", args, category_list.categories.to_vec());
                std::process::exit(1);
            //                return;
            // -v
            } else if char == 'v' {
                verbose = true;
            // -d
            } else if char == 'd' {
                dry_run = true;
            } else if char == 'p' {
                pretty = true;
            } else if char == 'e' {
                export = true;
            } else {
                help(format!("Unrecognized command line option in {0} : {1}", arg, char).as_str(), args, category_list.categories.to_vec());
                std::process::exit(1);
                //                return;
            }
        }

        first_arg_index += 1;
    }

    if pretty && !export {
        help("-p invalid without -e", args, category_list.categories.to_vec()); //TODO: help could print command line passed to it
        std::process::exit(1);
    }

    // first_arg should be the command code
    let first_arg = match args.get(first_arg_index) {
        Some(arg) => arg.to_lowercase(),
        None => String::new(),
    };

    if first_arg.is_empty() && !(export || dry_run) {
        help("No command specified", args, category_list.categories.to_vec());
        std::process::exit(1);
        //        return;
    }

    // find the invocable maching the argument from the list of invocable categories
    for category in category_list.categories.iter() {
        for invocable in category.invocables.iter() {
            if invocable.command_code == first_arg {
                // pass remaining command line arguments to the invocable
                let mut pass: Vec<String> = vec![];

                for arg in args.iter().skip(first_arg_index + 1) {
                    pass.push(wsl::wsl_path_or_self(arg, false));
                }

                if export {
                    if pretty {
                        println!("{}", serde_json::to_string_pretty(&invocable).unwrap());
                    } else {
                        println!("{}", serde_json::to_string(&invocable).unwrap());
                    }
                }

                Invoker {}.invoke(invocable, dry_run, verbose, pass);
                // avoid help() default below
                std::process::exit(0); // TODO: return result from command
            }
        }
    }

    if export {
        if pretty {
            println!("{}", serde_json::to_string_pretty(&category_list).unwrap());
        } else {
            println!("{}", serde_json::to_string(&category_list).unwrap());
        }
    }

    if export || dry_run {
        std::process::exit(0);
    }

    help(&format!("Command not recognized: {0}", first_arg), args, category_list.categories.to_vec());
    std::process::exit(2);
    // return
}

/// Writes the given message to STDOUT in a color other than the default.
fn color(msg: &str) {
    let mut terminal = term::stdout().unwrap();
    terminal.fg(term::color::BRIGHT_CYAN).unwrap();
    terminal.attr(term::Attr::Bold).unwrap();
    print!("{0}", msg);
    terminal.reset().unwrap();
}

//TODO: check for same command in multiple invocables accross all categories
//TODO: sysinternals not working?
// "get" => Invoker::cmd("echo"), // Windows File System explorer //TODO: rename echo?
//        "exp" => Invoker::exp("", &[]),    // Windows File System explorer
