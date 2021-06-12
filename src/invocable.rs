use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

// metadata about an invocable feature
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Invocable {
    pub command_code: String,   // command code for matching command line argument
    pub description: String,    // for help information
    pub command: String,        // path_to_program.exe, shell:desktop, etc.
    pub use_cmd: bool,          // cmd.exe [command]
    pub use_start: bool,        // cmd.exe start [command] //TODO: replace start with call except start/b
    pub background: bool,       // cmd.exe start /b [command]
    pub use_call: bool,         // cmd.exe start /b [command]
    pub use_explorer: bool,     // [cmd.exe] explorer.exe [/start | start /b]] [command]
    pub arguments: Vec<String>, // [explorer.exe | cmd.exe [/start | start /b] [command]] [arguments]
}

impl Eq for Invocable {}

impl Ord for Invocable {
    fn cmp(&self, other: &Self) -> Ordering {
        self.command_code.cmp(&other.command_code)
    }
}

impl PartialOrd for Invocable {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Invocable {
    fn eq(&self, other: &Self) -> bool {
        self.command_code == other.command_code
    }
}

impl Invocable {
    pub fn base(command_code: &'static str, command: &'static str, description: &'static str, args: &[&str]) -> Invocable {
        let mut inv = Invocable { command_code: command_code.to_string(), command: command.to_string(), description: description.to_string(), use_cmd: false, use_start: false, background: false, use_call: false, use_explorer: false, arguments: vec![] };

        for arg in args.iter() {
            inv.arguments.push(arg.to_string());
        }

        inv
    }

    pub fn bin_with(command_code: &'static str, command: &'static str, description: &'static str, args: &[&str]) -> Invocable {
        Invocable::base(command_code, command, description, args)
    }

    pub fn bin(command_code: &'static str, command: &'static str, description: &'static str) -> Invocable {
        Invocable::bin_with(command_code, command, description, &[])
    }

    pub fn exp_with(command_code: &'static str, command: &'static str, description: &'static str, args: &[&str]) -> Invocable {
        let mut inv = Invocable::base(command_code, command, description, args);
        inv.use_explorer = true;
        inv
    }

    pub fn exp(command_code: &'static str, command: &'static str, description: &'static str) -> Invocable {
        Invocable::exp_with(command_code, command, description, &[])
    }

    pub fn bkg_with(command_code: &'static str, command: &'static str, description: &'static str, args: &[&str]) -> Invocable {
        let mut inv = Invocable::base(command_code, command, description, args);
        inv.background = true;
        inv
    }

    pub fn bkg(command_code: &'static str, command: &'static str, description: &'static str) -> Invocable {
        Invocable::bkg_with(command_code, command, description, &[])
    }

    pub fn cmd_with(command_code: &'static str, command: &'static str, description: &'static str, args: &[&str]) -> Invocable {
        let mut inv = Invocable::base(command_code, command, description, args);
        inv.use_cmd = true;
        inv
    }

    pub fn cmd(command_code: &'static str, command: &'static str, description: &'static str) -> Invocable {
        Invocable::cmd_with(command_code, command, description, &[])
    }
}
