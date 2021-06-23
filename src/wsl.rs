//! The wink.wsl module contains a function for converting between Windows and Linux file system paths.

//TODO: this logic belongs in a library, not wink

/// Convert between Unix and Windows file paths.
/// The arg argument is the path to the file.
/// The unix argument indicates whether to convert that path to Unix or Windows.
// note: // unc path must start with \\; be careful not to replace \\ with / unintionally
//pub mod invocable;
//pub mod invocablecategory;
//pub mod invocablecategorylist;
//pub mod invoker;
pub mod inv;

pub fn wsl_path_or_self(arg: &str, unix: bool) -> String {
    let exists = std::path::Path::new(arg).exists();

    if exists && (is_wsl() && unix) || (is_windows() && !unix) {
        return arg.to_string();
    }

    let mut to_run = std::process::Command::new("wslpath");

    if unix {
        to_run.arg("-u");
    } else {
        to_run.arg("-w");
    }

    to_run.arg(arg);

    if let Ok(val) = to_run.output() {
        let result = String::from_utf8_lossy(&val.stdout).trim().to_string(); //.replace("\n", "");

        // if exaclty one non-whitespace line written to stdout
        if arg.len() != result.len()
            && (!result.is_empty())
            && result.as_bytes().iter().filter(|&&c| c == b'\n').count() < 1
        {
            return result;
        }
    }

    arg.to_string()
}

/// Return true if running under Windows (possibly a command shell) or WSL (possibly bash)
pub fn is_windows_or_wsl() -> bool {
    is_windows() || is_wsl()
}

/// Return true if running under Windows (possibly a command shell)
pub fn is_windows() -> bool {
    cfg!(target_os = "windows")
}

/// Return true if running under WSL (possibly a bash)
pub fn is_wsl() -> bool {
    match std::env::var("WSL_DISTRO_NAME") {
        Ok(_v) => true,
        Err(_e) => false,
    }
}

fn get_user_home() -> Result<String, Box<dyn std::error::Error>> {
    let key = if is_windows() { "USERPROFILE" } else { "HOME" };

    Ok(std::env::var(key)?)
}

fn get_user_home_default() -> String {
    match get_user_home() {
        Ok(h) => h,
        Err(_e) => String::new(),
    }
}

fn get_config_file_path(name: &str) -> String {
    if is_windows() {
        format!("{0}\\{1}", get_user_home_default(), name)
    } else {
        format!("{0}/.{1}", get_user_home_default(), name)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic]
    fn fail() {
        println!("wsl::tests::fail()");
        panic!("wsl::tests::fail()");
    }

    //fn get_user_home() -> Result<String, Box<dyn std::error::Error>> {
    //fn get_user_home_default() -> String {
    //fn get_config_file_path(name: &str) -> String {
    //wslpath_or_self

    #[test]
    fn get_user_home_default() {
        println!("wsl::tests::get_user_home_default()");
        //TODO:
    }

    #[test]
    fn is_windows_or_wsl() {
        println!("wsl::tests::is_windows_or_wsl()");
        assert_eq!(
            super::is_windows_or_wsl(),
            true,
            "Run the tests against the Linux binary under WSL."
        );
    }

    #[test]
    fn is_windows() {
        println!("wsl::tests::is_windows()");
        assert_eq!(
            super::is_windows(),
            false,
            "Run the tests against the Linux binary under WSL."
        );
    }

    #[test]
    fn is_wsl() {
        println!("wsl::tests::is_wsl()");
        assert_eq!(
            super::is_wsl(),
            true,
            "Run the tests against the Linux binary under WSL."
        );
    }
}
