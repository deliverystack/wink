//! An Invocable contains metadata about something that Windows can invoke,
//! whether an executable,
//! something that depends on a Windows process invoker such as explorer.exe,
//! something that depends on cmd.exe,
//! or something that depends on WSL bash.exe.

// for sorting by command code for usage information display
use std::cmp::Ordering;

// metadata about an invocable feature
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Invocable {
    /// The command code for the user to enter on the command line
    pub command_code: String, // command code for matching command line argument

    /// A description of what the command does
    pub description: String, // for help information

    /// The executable or Windows code such as shell:desktop or /path/to/script.
    pub command: String, // path_to_program.exe, shell:desktop, etc.

    /// If true, the invocable depends on cmd.exe.
    pub use_cmd: bool, // cmd.exe [command]

    /// If true, the invocable depends on cmd.exe and uses start.
    pub use_start: bool, // cmd.exe start [command] //TODO: replace start with call except start/b

    /// If true, the invocable depnds on cmd.exe and uses start /b.
    pub background: bool, // cmd.exe start /b [command]

    /// If true, the invocable depends on cmd.exe and uses call.
    pub use_call: bool, // cmd.exe call <command> [arguments]

    /// If true, the invocable depends on explorer.exe.
    pub use_explorer: bool, // [cmd.exe] explorer.exe [/start | start /b]] <command> [arguments]

    /// If true, the invocable depends on bash.exe.
    pub use_bash: bool, // bash.exe -c <command> [arguments]

    /// Arguments to pass on the command line before those provided by the user.
    pub arguments: Vec<String>, // [explorer.exe | cmd.exe [/start | start /b] <command> [arguments]
}

/// For sorting.
impl Eq for Invocable {}

/// For sorting.
impl Ord for Invocable {
    fn cmp(&self, other: &Self) -> Ordering {
        self.command_code.cmp(&other.command_code)
    }
}

/// For sorting.
impl PartialOrd for Invocable {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// For sorting.
impl PartialEq for Invocable {
    fn eq(&self, other: &Self) -> bool {
        self.command_code == other.command_code
    }
}

impl Invocable {
    /// Construct and return a default invocable.
    fn base(command_code: &'static str, command: &'static str, description: &'static str, args: &[&str]) -> Invocable {
        let mut inv = Invocable { command_code: command_code.to_string(), command: command.to_string(), description: description.to_string(), use_cmd: false, use_start: false, background: false, use_call: false, use_explorer: false, use_bash: false, arguments: vec![] };

        for arg in args.iter() {
            inv.arguments.push(arg.to_string());
        }

        inv
    }

    /// Construct and return an invocable that calls an executable directly without explorer.exe, cmd.exe, or bash.exe.
    /// Accepts command line arguments specified by invocable construction.
    pub fn bin_with(command_code: &'static str, command: &'static str, description: &'static str, args: &[&str]) -> Invocable {
        Invocable::base(command_code, command, description, args)
    }

    /// Return an Invocable that calls an executable directly without explorer.exe, cmd.exe, or bash.exe.
    /// Convenience method for common case of no configured command line arguments; ; user may specify command line arguments for invocable.
    pub fn bin(command_code: &'static str, command: &'static str, description: &'static str) -> Invocable {
        Invocable::bin_with(command_code, command, description, &[])
    }

    /// Return an Invocable that uses explorer.exe.
    /// Accepts command line arguments specified by invocable construction.
    pub fn exp_with(command_code: &'static str, command: &'static str, description: &'static str, args: &[&str]) -> Invocable {
        let mut inv = Invocable::base(command_code, command, description, args);
        inv.use_explorer = true;
        inv
    }

    /// Return an Invocable that uses explorer.exe. Convenience method for no command line arguments.
    /// Convenience method for common case of no configured command line arguments; ; user may specify command line arguments for invocable.
    pub fn exp(command_code: &'static str, command: &'static str, description: &'static str) -> Invocable {
        Invocable::exp_with(command_code, command, description, &[])
    }

    /// Return an invocable that uses cmd.exe start /b.
    /// Accepts command line arguments specified by invocable construction.
    pub fn bkg_with(command_code: &'static str, command: &'static str, description: &'static str, args: &[&str]) -> Invocable {
        let mut inv = Invocable::base(command_code, command, description, args);
        inv.background = true;
        inv
    }

    /// Return an invocable that uses cmd.exe start /b.
    /// Convenience method for common case of no configured command line arguments; ; user may specify command line arguments for invocable.
    pub fn bkg(command_code: &'static str, command: &'static str, description: &'static str) -> Invocable {
        Invocable::bkg_with(command_code, command, description, &[])
    }

    /// Return an invocable that uses cmd.exe.
    /// Accepts command line arguments specified by invocable construction.
    pub fn cmd_with(command_code: &'static str, command: &'static str, description: &'static str, args: &[&str]) -> Invocable {
        let mut inv = Invocable::base(command_code, command, description, args);
        inv.use_cmd = true;
        inv
    }

    /// Return an invocable that uses cmd.exe.
    /// Convenience method for common case of no configured command line arguments; ; user may specify command line arguments for invocable.
    pub fn cmd(command_code: &'static str, command: &'static str, description: &'static str) -> Invocable {
        Invocable::cmd_with(command_code, command, description, &[])
    }

    /// Return an invocable that uses bash.exe.
    /// Accepts command line arguments specified by invocable construction.
    pub fn sh_with(command_code: &'static str, command: &'static str, description: &'static str, args: &[&str]) -> Invocable {
        let mut inv = Invocable::base(command_code, command, description, args);
        inv.use_bash = true;
        inv
    }

    /// Return an invocable that uses bash.exe.
    /// Convenience method for common case of no configured command line arguments; ; user may specify command line arguments for invocable.
    pub fn sh(command_code: &'static str, command: &'static str, description: &'static str) -> Invocable {
        Invocable::sh_with(command_code, command, description, &[])
    }
}
