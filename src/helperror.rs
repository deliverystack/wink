#[derive(Debug)]
pub struct HelpError {
    pub message: String,
}

impl HelpError {
    pub fn new(msg: String) -> HelpError {
        HelpError { message: msg }
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
