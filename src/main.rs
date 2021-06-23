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

use wink::wsl::inv::invocablecategorylist::InvocableCategoryList;
use wink::wsl::inv::invoker::Invoker;

/// The main() function of the program accepts command line arguments through env::args.collect()
/// rather than as parameters.
fn main() {
    let config = wink::WinkConfig::get_from_cmd_line_args();

    // categories contain lists of invocables that map command codes to commands
    let category_list = InvocableCategoryList::get();

    if !config.help_msg.is_empty() {
        wink::help(
            &config.help_msg.to_string(),
            config,
            category_list.categories,
        );
        std::process::exit(1);
    }

    // if not running under WSL or Windows, it should not be possible to run Windows commands
    if !wink::wsl::is_windows_or_wsl() {
        wink::help("Runs only under Windows and Windows Subsystem for Linux (WSL). Define WSL_DISTRO_NAME environment variable to override.", config, category_list.categories);
        std::process::exit(2);
    }

    if config.pretty_print && !config.export {
        wink::help("-p invalid without -e", config, category_list.categories);
        std::process::exit(3);
    }

    if config.command_code.is_empty() && !(config.export || config.dry_run) {
        wink::help("No command specified", config, category_list.categories);
        std::process::exit(4);
    }

    if let Some(invocable) = category_list.get_invocable(&config.command_code) {
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

    wink::help(
        &format!("Command not recognized: {0}", config.command_code),
        config,
        category_list.categories,
    );
    std::process::exit(5);
}
