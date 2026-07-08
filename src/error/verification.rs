use std::fmt;

#[derive(Debug)]
pub struct VerificationError {
    pub exit_code: u8,
    pub errors: Vec<String>,
}

impl fmt::Display for VerificationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Verification failed (exit code {}): ", self.exit_code)?;
        if self.errors.is_empty() {
            write!(f, "No error details available")?;
        } else {
            for (i, err) in self.errors.iter().enumerate() {
                if i > 0 {
                    write!(f, "; ")?;
                }
                write!(f, "{}", err)?;
            }
        }
        Ok(())
    }
}

impl std::error::Error for VerificationError {}

pub const NO_SUCH_FILE: u8 = 1;
pub const PARSE_ERROR: u8 = 2;
pub const BLOCKED_SYNTAX_ERROR: u8 = 3;
