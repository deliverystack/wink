//! This library contains the WinkConfig struct that represents
//! command line options passed to the wink command.

pub mod helperror; // /src/helperror.rs defines helperror::HelpError
pub mod winkconfig; // /src/winkconfig.rs defines winkconfig::WinkConfig
pub mod wsl; // /wsl.rs defines the contents of wsl::

use crate::wsl::inv::invocablecategory::InvocableCategory; // /src/wsl/inv/invocablecategory.rs
use crate::wsl::inv::invocablecategorylist::InvocableCategoryList; // /src/wsl/inv/invocablecategorylist.rs
use crate::wsl::inv::invoker::Invoker; // /src/wsl/inv/invoker.rs

pub fn run(config: crate::winkconfig::WinkConfig, category_list: InvocableCategoryList) -> i32 {
    // -> Result<u8, Box<dyn std::error::Error>> {
    // categories contain lists of invocables that map command codes to commands
    if let Some(invocable) = category_list.get_invocable(&config.command_code) {
        if config.export {
            if config.pretty_print {
                println!("{}", serde_json::to_string_pretty(&invocable).unwrap());
            } else {
                println!("{}", serde_json::to_string(&invocable).unwrap());
            }
        }

        let invoker = Invoker {};
        invoker.invoke(invocable, config.dry_run, config.verbose, config.cmd_args);
        return 0;
    } else if config.export && config.command_code.is_empty() {
        if config.pretty_print {
            println!("{}", serde_json::to_string_pretty(&category_list).unwrap());
        } else {
            println!("{}", serde_json::to_string(&category_list).unwrap());
        }

        return 0;
    } else if (config.command_code.is_empty() || !config.export) && config.dry_run {
        return 0;
    }

    help(
        &format!("Command not recognized: {0}", config.command_code),
        config,
        category_list.categories,
    )
}

/// The help() function renders usage information about the wink command to stdout.
/// The msg argument is a message indicating why the command rendered usage information.
/// The args argument is the command line including the invoked command (wink) and command line arguments.
/// The categories argument contains lists of invocables used to render usage information.
pub fn help(
    msg: &str,
    config: crate::winkconfig::WinkConfig,
    mut categories: Vec<InvocableCategory>,
) -> i32 {
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
    println!(
        "            -h help (display this {0} command usage information)",
        config.cmd_name
    );
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

    1
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
    #[should_panic]
    /// this test renders the path to the parent of this tests module
    fn it_has_a_path() {
        panic!("it_has_a_path intentional panic to render path to parent of this tests module.")
    }
}
