use crate::error::{
    result::{StockTrekError, StockTrekResult},
    verification::{VerificationError, NO_SUCH_FILE},
};

pub struct FileExistenceVerifier;

impl FileExistenceVerifier {
    pub fn new() -> Self {
        Self
    }
    pub fn verify(&self, path: String) -> StockTrekResult<String> {
        match std::fs::read_to_string(path) {
            Err(e) => Err(StockTrekError::Verification(VerificationError {
                exit_code: NO_SUCH_FILE,
                errors: vec![e.to_string()],
            })),
            Ok(contents) => Ok(contents),
        }
    }
}

impl Default for FileExistenceVerifier {
    fn default() -> Self {
        FileExistenceVerifier::new()
    }
}
