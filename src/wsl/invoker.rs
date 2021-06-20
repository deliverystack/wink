//! Invoker contains a function that creates and invokes a command line
//! based on metadata in an Invocable and command line arguments.

//TODO: is this the best way to reference the Invocable struct and the wsl_path_or_self() function?

use crate::wsl::invocable::Invocable;
use crate::wsl::wsl_path_or_self;

pub struct Invoker {}

impl Invoker {
    /// This function creates a command line from the specified invocable and args,
    /// writes that command line to stdout if verbose is true,
    /// and invokes that command line.
    //TODO:     invoker::Invoker::invoke(invocable, dry_run, verbose, pass); just doesn't look right to document.
    pub fn invoke(&self, invocable: &Invocable, dry_run: bool, verbose: bool, args: Vec<String>) -> String {
        // create three constants for substituting tokens in command paths
        let results = std::process::Command::new("cmd.exe").arg("/c").arg("echo").arg("%USERPROFILE%").output().expect("failed to execute process");
        let userpath: String = match results.status.code() {
            Some(0) => wsl_path_or_self(String::from_utf8_lossy(&results.stdout).trim(), false),
            _ => String::new(),
        };

        let results = std::process::Command::new("cmd.exe").arg("/c").arg("echo").arg("%ProgramFiles%").output().expect("failed to execute process");
        let pf64: String = match results.status.code() {
            Some(0) => wsl_path_or_self(String::from_utf8_lossy(&results.stdout).trim(), false),
            _ => String::new(),
        };

        let results = std::process::Command::new("cmd.exe").arg("/c").arg("echo").arg("%ProgramFiles(x86)%").output().expect("failed to execute process");
        let pf86 = match results.status.code() {
            Some(0) => wsl_path_or_self(String::from_utf8_lossy(&results.stdout).trim(), false),
            _ => String::new(),
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
        let maybe_executable = &wsl_path_or_self(&invocable.command.replace("$pf64", &pf64).replace("$pf86", &pf86).replace("$userpath", &userpath).replace("$syslive", "\\\\live.sysinternals.com\\tools\\"), !cfg!(target_os = "windows"));

        // if directed to use cmd.exe or start or start /b, then use cmd.exe /c
        // else if directed to use explorer.exe, then use explorer.exe
        // otherwise invoke the executable directly
        if invocable.use_cmd || invocable.use_start || invocable.background {
            cmd = "cmd.exe";
        } else if invocable.use_bash {
            cmd = "bash.exe"
        } else if invocable.use_explorer {
            cmd = "explorer.exe";
        } else {
            cmd = &maybe_executable;
        }

        command_line.push_str(cmd);
        command_line.push(' ');

        // the Command object to invoke the command line
        let mut torun = std::process::Command::new(String::from(cmd));

        // /wait and /c for cmd.exe
        if invocable.use_cmd {
            if !invocable.background {
                torun.arg("/wait");
                command_line.push_str("/wait ");
            }

            torun.arg("/c");
            command_line.push_str("/c ");
        }

        if invocable.use_bash {
            //TODO:  if invocable.background
            torun.arg("-c");
            command_line.push_str("-c ");
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
        if (invocable.use_cmd || invocable.use_start || invocable.background || invocable.use_explorer || invocable.use_bash) && !invocable.command.is_empty() {
            let command: &String = &wsl_path_or_self(&invocable.command.replace("$pf64", &pf64).replace("$pf86", &pf86).replace("$userpath", &userpath).replace("$syslive", "\\\\live.sysinternals.com\\tools\\"), invocable.use_bash);
            torun.arg(command);
            command_line.push_str(command);
            command_line.push(' ');
        }

        let mut bash_command = String::new();

        //TODO: for bash.exe, it seems that the entire command line should appear as a properly quoted string.
        // bash.exe -c wslpath -u C:/temp does not work, but bash.exe -c "wslpath -u C:/temp" does
        // add arguments from command configuration to command line
        for arg in invocable.arguments.iter() {
            let param: &String = &wsl_path_or_self(arg, invocable.use_bash);

            if invocable.use_bash {
                bash_command = format!("{0}{1} ", bash_command, param); //TOOD: quote?
            } else {
                torun.arg(param);
                command_line.push_str(param);
                command_line.push(' ');
            }
        }

        // append args from called command line to command line
        for arg in args.iter() {
            let param: &String = &wsl_path_or_self(arg, invocable.use_bash);

            if invocable.use_bash {
                bash_command = format!("{0}{1} ", bash_command, param); //TOOD: quote?
            } else {
                torun.arg(param);
                command_line.push_str(param);
                command_line.push(' ');
            }
        }

        if !bash_command.is_empty() {
            bash_command = bash_command.trim().to_string();
            command_line.push_str(&bash_command);
            torun.arg(bash_command);
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

                if !err.is_empty() {
                    eprintln!("{}", err);
                }

                let out = String::from_utf8_lossy(&results.stdout);

                if !out.is_empty() {
                    println!("{}", out);
                }
            }
        }

        command_line
    }
}

//TODO: shell:::{7b81be6a-ce2b-4676-a29e-eb907a5126c5}", // ms-settings:network-status
//TODO:        self.add(Invocable::exp("eacur", "ms-settings:easeofaccess-cursorandpointersize", "Ease of Access cursor and pointer size")); //TODO: fail
//TODO:        self.add(Invocable::exp("eapoint", "ms-settings:easeofaccess-MousePointer", "Ease of Access mouse pointer settings")); //TODO: fail
