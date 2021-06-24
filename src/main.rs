//TODO: check for same command in multiple invocables accross all categories
//TODO: sysinternals not working?
// "get" => Invoker::cmd("echo"), // Windows File System explorer //TODO: rename echo?
//        "exp" => Invoker::exp("", &[]),    // Windows File System explorer

//! wink is a command line tool that provides access to Windows and Windows Subsystem for Linux (WSL) features and programs.
//! wink provides a simple interface for invoking almost any Windows or WSL feature
//! using cmd.exe /c, using explorer.exe, using bash.exe, or by invoking the executable directly.
//! wink uses cargo, so you can use cargo build to build wink. You can also use the wince script to build wink.
//! Run wink with no command line parameters to get usage information.

/// The main() function of the program accepts command line arguments through env::args.collect()
/// rather than as parameters.
fn main() {
    // get a list of categories containing invocable commands
    // defined in /wsl/inv/invocablecategorylist.rs
    let category_list = wink::wsl::inv::invocablecategorylist::InvocableCategoryList::get();
    let config = wink::winkconfig::WinkConfig::new(std::env::args().collect()); // parse command line arguments

    // if successful parsing command line, the Ok enum value contains the WinkConfig
    // otherwise, Err enum value contains the WinkConfig and a HelpError
    std::process::exit(match config {
        Ok(config) => wink::run(config, category_list), // what if this raises an error
        Err((config, e)) => wink::help(&e.to_string(), config, category_list.categories),
    });
}
