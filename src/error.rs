use std::fmt;

// Program-level exit codes (0-9)
pub const EXIT_SUCCESS: i32 = 0;
pub const EXIT_PROGRAM_ERROR: i32 = 1;

// Exit codes for specific errors in upv-cli (10-19)
//pub const EXIT_UPV_ERROR: i32 = 10;
pub const EXIT_UPV_VPN_ERROR: i32 = 11;
pub const EXIT_UPV_DRIVE_ERROR: i32 = 12;
pub const EXIT_UPV_DRIVE_IN_USE_ERROR: i32 = 13;

#[derive(Debug)]
pub struct UpvError {
    pub message: String,
    pub exit_code: i32,
}

impl UpvError {
    pub fn new(message: impl Into<String>, exit_code: i32) -> Self {
        Self {
            message: message.into(),
            exit_code,
        }
    }
}

impl fmt::Display for UpvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for UpvError {}