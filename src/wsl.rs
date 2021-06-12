// convert between Unix and Windows file paths
// when running in Windows, return arg.
// when running under Bash, invoke wslpath.
// unix true will pass -u (convert Windows path to Unix)
// unix false will pass -w (convert Unix path to Windows)

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
            Some(0) => {
                let output = &String::from_utf8_lossy(&results.stdout).to_owned().to_string();

                if output.starts_with('\\') {
                    // unc path must start with \\
                    return String::from(output).replace("\n", "");
                }

                String::from(output).replace("\n", "")
            }

            _ => arg.to_string(),
        }
    }
}
