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
    pub fn new(reason: String) -> Self {
        Self {
            chunk_code: "Unknown".to_string(),
            reason,
        }
    }

    pub fn new_with_id(id: String, reason: String) -> Self {
        Self {
            chunk_code: id,
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

#[derive(Debug)]
pub struct FieldParseError {
    pub(crate) chunk_code: String,
    pub(crate) field_name: String,
    pub(crate) position: usize,
    pub(crate) reason: String,
}

impl From<FieldParseError> for PyErr {
    fn from(value: FieldParseError) -> Self {
        PyValueError::new_err(value.reason)
    }
}

impl From<FieldParseError> for () {
    fn from(_value: FieldParseError) -> Self {}
}

impl Display for FieldParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Unable to parse {} chunk field {}: {}",
            self.chunk_code, self.field_name, self.reason
        )
    }
}

impl Error for FieldParseError {}
