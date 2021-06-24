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

/// The main() function of the program accepts command line arguments through env::args.collect()
/// rather than as parameters.
fn main() {
    let category_list = InvocableCategoryList::get();
    let config = wink::WinkConfig::new(std::env::args().collect());

    std::process::exit(match config {
        Ok(config) => wink::run(config, category_list),
        Err((config, e)) => wink::help(&e.to_string(), config, category_list.categories),
    });
}
