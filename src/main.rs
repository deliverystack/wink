//TODO: check for same command in multiple invocables accross all categories
//TODO: sysinternals not working?
// "get" => Invoker::cmd("echo"), // Windows File System explorer //TODO: rename echo?
//        "exp" => Invoker::exp("", &[]),    // Windows File System explorer

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
//use lib;

use wsl::inv::{
    invocablecategory::InvocableCategory, invocablecategorylist::InvocableCategoryList,
    invoker::Invoker,
};

/// The help() function renders usage information about the wink command to stdout.
/// The msg argument is a message indicating why the command rendered usage information.
/// The args argument is the command line including the invoked command (wink) and command line arguments.
/// The categories argument contains lists of invocables used to render usage information.
fn help(msg: &str, config: wink::WinkConfig, mut categories: Vec<InvocableCategory>) {
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

/// The main() function of the program accepts command line arguments through env::args.collect()
/// rather than as parameters.
fn main() {
    let config = wink::WinkConfig::get_from_cmd_line_args();

    // categories contain lists of invocables that map command codes to commands
    let category_list = InvocableCategoryList::get();

    if !config.help_msg.is_empty() {
        help(
            &config.help_msg.to_string(),
            config,
            category_list.categories,
        );
        std::process::exit(2);
    }

    // if not running under WSL or Windows, it should not be possible to run Windows commands
    if !crate::wsl::is_windows_or_wsl() {
        help("Runs only under Windows and Windows Subsystem for Linux (WSL). Define WSL_DISTRO_NAME environment variable to override.", config, category_list.categories);
        std::process::exit(2);
    }

    if config.pretty_print && !config.export {
        help("-p invalid without -e", config, category_list.categories);
        std::process::exit(3);
    }

    if config.command_code.is_empty() && !(config.export || config.dry_run) {
        help("No command specified", config, category_list.categories);
        std::process::exit(4);
    }

    // find the invocable maching the argument from the list of invocable categories
    for category in category_list.categories.iter() {
        for invocable in category.invocables.iter() {
            if invocable.command_code == config.command_code {
                // pass remaining command line arguments to the invocable

                if config.export {
                    if config.pretty_print {
                        println!("{}", serde_json::to_string_pretty(&invocable).unwrap());
                    } else {
                        println!("{}", serde_json::to_string(&invocable).unwrap());
                    }
                }

                Invoker {}.invoke(invocable, config.dry_run, config.verbose, config.cmd_args);
                std::process::exit(0); // TODO: return result from invoking command
            }
        }
    }

    if config.export && config.command_code.is_empty() {
        if config.pretty_print {
            println!("{}", serde_json::to_string_pretty(&category_list).unwrap());
        } else {
            println!("{}", serde_json::to_string(&category_list).unwrap());
        }

        std::process::exit(0);
    }

    if (config.command_code.is_empty() || !config.export) && config.dry_run {
        std::process::exit(0);
    }

    help(
        &format!("Command not recognized: {0}", config.command_code),
        config,
        category_list.categories,
    );
    std::process::exit(5);
}

/// Writes the given message to STDOUT in a color other than the default.
fn color(msg: &str) {
    let mut terminal = term::stdout().unwrap();
    terminal.fg(term::color::BRIGHT_CYAN).unwrap();
    terminal.attr(term::Attr::Bold).unwrap();
    print!("{0}", msg);
    terminal.reset().unwrap();
}
