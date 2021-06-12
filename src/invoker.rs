use std::process::Command;

use crate::invocable;
use crate::wsl;

//TODO: is this the right way to define a container for static methods?
// creates and invokes command lines from invocables
pub struct Invoker {}

impl Invoker {
    // create and command line, output if verbose, run unless dry
    pub fn invoke(invocable: &invocable::Invocable, dry_run: bool, verbose: bool, args: Vec<String>) -> String {
        // create three constants for substituting tokens in command paths
        let results = Command::new("cmd.exe").arg("/c").arg("echo").arg("%USERPROFILE%").output().expect("failed to execute process");
        let userpath: String = match results.status.code() {
            Some(0) => wsl::wsl_path_or_self(String::from_utf8_lossy(&results.stdout).trim(), false),
            _ => String::from(""),
        };

        let results = Command::new("cmd.exe").arg("/c").arg("echo").arg("%ProgramFiles%").output().expect("failed to execute process");
        let pf64: String = match results.status.code() {
            Some(0) => wsl::wsl_path_or_self(String::from_utf8_lossy(&results.stdout).trim(), false),
            _ => String::from(""),
        };

        let results = Command::new("cmd.exe").arg("/c").arg("echo").arg("%ProgramFiles(x86)%").output().expect("failed to execute process");
        let pf86 = match results.status.code() {
            Some(0) => wsl::wsl_path_or_self(String::from_utf8_lossy(&results.stdout).trim(), false),
            _ => String::from(""),
        };

        // for -v [verbose] command line option
        let mut command_line = String::new();

        // the command to invoke - cmd.exe, explorer.exe, or the executable specified in the command field of the invocable.
        let cmd: &str;

        // if directed to use cmd.exe or start or start /b, then use cmd.exe /c
        // else if directed to use explorer.exe, then use explorer.exe
        // otherwise invoke the executable directly
        // this would be the executable to invoke
        //TODO: create maybe_executable in else block below instead of here; maybe requires cmd to be String?
        let maybe_executable = &wsl::wsl_path_or_self(&invocable.command.replace("$pf64", &pf64).replace("$pf86", &pf86).replace("$userpath", &userpath).replace("$syslive", "\\\\live.sysinternals.com\\tools\\"), !cfg!(target_os = "windows"));

        // if directed to use cmd.exe or start or start /b, then use cmd.exe /c
        // else if directed to use explorer.exe, then use explorer.exe
        // otherwise invoke the executable directly
        if invocable.use_cmd || invocable.use_start || invocable.background {
            cmd = "cmd.exe";
        } else if invocable.use_explorer {
            cmd = "explorer.exe";
        } else {
            cmd = &maybe_executable;
        }

        command_line.push_str(cmd);
        command_line.push(' ');

        // the Command object to invoke the command line
        let mut torun = Command::new(String::from(cmd));

        // /wait and /c for cmd.exe
        if invocable.use_cmd {
            if !invocable.background {
                torun.arg("/wait");
                command_line.push_str("/wait ");
            }

            torun.arg("/c");
            command_line.push_str("/c ");
        }

        // both start and start /b require start
        if invocable.use_start || invocable.background {
            torun.arg("start");
            command_line.push_str("start ");
        }

        // start /b
        if invocable.background {
            torun.arg("/b");
            command_line.push_str("/b ");
        };

        // untested; runs without creating a new environment?
        if invocable.use_call {
            torun.arg("call");
            command_line.push_str("call ");
        }

        // if executable specified with cmd.exe then add windows path to executable to command line
        if (invocable.use_cmd || invocable.use_start || invocable.background || invocable.use_explorer) && !invocable.command.is_empty() {
            let command: &String = &wsl::wsl_path_or_self(&invocable.command.replace("$pf64", &pf64).replace("$pf86", &pf86).replace("$userpath", &userpath).replace("$syslive", "\\\\live.sysinternals.com\\tools\\"), false);
            torun.arg(command);
            command_line.push_str(command);
            command_line.push(' ');
        }

        // add arguments from command configuration to command line
        for arg in invocable.arguments.iter() {
            let param: &String = &wsl::wsl_path_or_self(arg, false);
            torun.arg(param);
            command_line.push_str(param);
            command_line.push(' ');
        }

        // append args from called command line to command line
        for arg in args.iter() {
            let param: &String = &wsl::wsl_path_or_self(arg, false);
            torun.arg(param);
            command_line.push_str(param);
            command_line.push(' ');
        }

        if verbose {
            println!("{}", command_line);
        }

        if !dry_run {
            if invocable.background {
                let _discard = torun.status();
            } else {
                let results = torun.output().expect("failed to execute process");
                let err = String::from_utf8_lossy(&results.stderr);

                if err != "" {
                    eprintln!("{}", err);
                }

                let out = String::from_utf8_lossy(&results.stdout);

                if out != "" {
                    println!("{}", out);
                }
            }
        }

        command_line
    }
}
