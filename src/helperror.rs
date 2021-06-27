#[derive(Debug)]
pub struct HelpError {
    pub message: String,
    pub help_requested: bool,
}

impl HelpError {
    pub fn new(message: String, help_requested: bool) -> HelpError {
        HelpError {
            message,
            help_requested,
        }
    }
}

impl std::fmt::Display for HelpError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for HelpError {
    fn description(&self) -> &str {
        &self.message
    }
}
