//! The wink.wsl module contains a function for converting between Windows and Linux file system paths.

//TODO: this logic belongs in a library, not wink

/// Convert between Unix and Windows file paths.
/// The arg argument is the path to the file.
/// The unix argument indicates whether to convert that path to Unix or Windows.
/// Example usage:
/// ```
/// let results = Command::new("cmd.exe").arg("/c").arg("echo").arg("%USERPROFILE%").output().expect("failed to execute process");
/// let userpath: String = match results.status.code() {
///     Some(0) => wsl::wsl_path_or_self(String::from_utf8_lossy(&results.stdout).trim(), false /*unix*/ ),
///    _ => String::from(""),
/// };
/// ```
// note: // unc path must start with \\; be careful not to replace \\ with / unintionally
pub fn wsl_path_or_self(arg: &str, unix: bool) -> String {
    if (cfg!(target_os = "windows") && !unix) || (unix && arg.starts_with('/')) {
        arg.to_string()
    } else {
        let mut to_run = std::process::Command::new("wslpath");
        if unix {
            to_run.arg("-u");
        } else {
            to_run.arg("-w");
        }
        to_run.arg(arg);
        let results = to_run.output().expect("failed to execute process");
        match results.status.code() {
            Some(0) => String::from(&String::from_utf8_lossy(&results.stdout).to_owned().to_string()).replace("\n", ""),
            _ => arg.to_string(),
        }
    }
}
