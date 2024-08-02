use pyo3::exceptions::PyValueError;
use pyo3::PyErr;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct ChunkParseError {
    pub(crate) chunk_code: String,
    pub(crate) reason: String,
}

impl From<ChunkParseError> for PyErr {
    fn from(value: ChunkParseError) -> Self {
        PyValueError::new_err(value.reason)
    }
}

impl From<ChunkParseError> for () {
    fn from(_value: ChunkParseError) -> Self {}
}

impl ChunkParseError {
    pub fn new(chunk_code: Option<&str>, reason: String) -> Self {
        ChunkParseError {
            chunk_code: chunk_code.unwrap_or("Unknown").to_string(),
            reason,
        }
    }
}

impl Display for ChunkParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Unable to parse {} chunk: {}",
            self.chunk_code, self.reason
        )
    }
}

impl Error for ChunkParseError {}
