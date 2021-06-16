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
///    _ => String::new(),
/// };
/// ```
// note: // unc path must start with \\; be careful not to replace \\ with / unintionally
pub fn wsl_path_or_self(arg: &str, unix: bool) -> String {
    //    if (!unix || arg.starts_with('/')) {
    //        // && arg.starts_with('/')) {
    let mut to_run = std::process::Command::new("wslpath");
    if unix {
        to_run.arg("-u");
    } else {
        to_run.arg("-w");
    }

    to_run.arg(arg);

    //TODO: check whether error control works when wslpath fails (for cmd.exe)
    // or when path does not exist
    //TODO: use bash.exe to invoke wslpath for cmd.exe
    if let Ok(val) = to_run.output() {
        let result = String::from_utf8_lossy(&val.stdout).trim().to_string(); //.replace("\n", "");

        if !result.is_empty() && result.as_bytes().iter().filter(|&&c| c == b'\n').count() < 1 { // if more than one line, must be an error message
            return result;
        }
    }
    //    }

    arg.to_string()
}
