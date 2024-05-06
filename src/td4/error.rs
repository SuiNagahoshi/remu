#[derive(Debug)]
pub struct EmulatorError {
    message: String,
}

impl EmulatorError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}
