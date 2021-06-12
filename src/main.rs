use regex::Regex;
use std::env;

mod file;
mod invocable;
mod invocablecategory;
mod invocablecategorylist;
mod invoker;
mod wsl;

fn help(msg: &str, args: Vec<String>, mut categories: Vec<invocablecategory::InvocableCategory>) {
    let cmd = Regex::new(r".*[\\/](?P<name>[^\\/]+$)").unwrap().replace_all(args[0].as_str(), "$name");

    println!();
    println!("-----------------------------------------------------------------------------");
    println!("{0} : access Windows features : {1}", cmd, msg);
    println!(
        "\
-----------------------------------------------------------------------------
{0:>12} exp                explorer.exe
-----------------------------------------------------------------------------
{0:>12} exp <file.ext>     Set/open default application for extension
{0:>12} exp <shell:sendto> Invoke command code (replace <shell:sendto>)
-----------------------------------------------------------------------------

{0:>12} cmd                cmd.exe /c
-----------------------------------------------------------------------------
{0:>12} cmd <cmd> [args]   Invoke Windows console command line
{0:>12} cmd echo %PATH%    Display Windows environment variable
-----------------------------------------------------------------------------

-----------------------------------------------------------------------------
{0:>12} code [args]        See command code tables below
-----------------------------------------------------------------------------",
        cmd
    );

    let mut count = 0;
    categories.sort();

    for mut category in categories {
        println!("\n{0}\n-----------------------------------------------------------------------------", category.name);
        category.invocables.sort();

        for invocable in category.invocables {
            let mut desc = invocable.description;

            if desc.is_empty() {
                desc = invocable.command;
            }

            println!("{:>31} {}", invocable.command_code, desc);
            count += 1;
        }
    }

    println!("\n{0:>12} : {1} known command codes\n", cmd, count);
    println!("{0:>12} : access Windows features : {1}\n", cmd, msg);
    println!("{0:>12} [opts] <code> [arguments]     : invoke command by command code", cmd);
    println!("            -d dry (do not execute)");
    println!("            -e export (dump configuraiton JSON)");
    println!("            -v verbose (print command line)");
    println!("{0} help                          : display command usage information", cmd);

    if cfg!(target_os = "windows") {
        println!("{0} help | find /i \"text\"         : identify command code matching text", cmd);
    } else {
        println!("{0} help | grep -i \"text\"         : identify command code matching text", cmd);
    }
}

fn main() {
    // command line arguments
    let args: Vec<String> = env::args().collect();

    // categories contain lists of invocables that map command codes to commands
    let category_list = invocablecategorylist::InvocableCategoryList::get();

    // if not running under WSL or Windows, it should not be possible to run Windows commands
    match env::var("WSL_DISTRO_NAME") {
        Ok(_e) => {}
        Err(_e) => {
            if !cfg!(target_os = "windows") {
                help("Runs only under Windows and Windows Subsystem for Linux (WSL). Define WSL_DISTRO_NAME environment variable to override.", args, category_list.categories.to_vec());
                return;
            }
        }
    }

    let mut dry_run: bool = false; // -d command line option
    let mut verbose: bool = false; // -v command line option
    let mut export: bool = false; // -e command line option
    let mut first_arg_index = 1; // number of processed command line arguments (first is command name)

    for arg in args.iter().skip(1) {
        let prefix: char = arg.to_lowercase().chars().next().unwrap();

        if arg == "help" {
            help("Help requested", args, category_list.categories.to_vec());
            return;
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
                return;
            // -v
            } else if char == 'v' {
                verbose = true;
            // -d
            } else if char == 'd' {
                dry_run = true;
            } else if char == 'e' {
                export = true;
            } else {
                help(format!("Unrecognized command line option in {0} : {1}", arg, char).as_str(), args, category_list.categories.to_vec());
                return;
            }
        }

        first_arg_index += 1;
    }

    if export {
        println!("{}", serde_json::to_string(&category_list).unwrap());
    }

    // first_arg should be the command code
    let first_arg = match args.get(first_arg_index) {
        Some(arg) => arg.to_lowercase(),
        None => String::new(),
    };

    if first_arg.is_empty() {
        help("No command specified", args, category_list.categories.to_vec());
        return;
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

                invoker::Invoker::invoke(invocable, dry_run, verbose, pass);
                return; // avoid help() default below //TODO: return 0, return 1 for help, return 2 below
            }
        }
    }

    help(&format!("Command not recognized: {0}", first_arg), args, category_list.categories.to_vec());
}

// stikynot
//TODO: check for same command in multiple invocables accross all categories
//TODO: sysinternals not working?
// "get" => Invoker::cmd("echo"), // Windows File System explorer //TODO: rename echo?
//        "exp" => Invoker::exp("", &[]),    // Windows File System explorer
