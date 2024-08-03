use pyo3::exceptions::PyValueError;
use pyo3::PyErr;
use std::error::Error;
use std::fmt::Display;

pub trait ChunkError: Into<PyErr> + Error {}

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
impl ChunkError for ChunkParseError {}

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
impl ChunkError for FieldParseError {}

#[derive(Debug)]
pub struct IncorrectChunkError {
    pub(crate) expected_chunk_code: String,
    pub(crate) actual_chunk_code: String,
}

impl From<IncorrectChunkError> for PyErr {
    fn from(value: IncorrectChunkError) -> Self {
        PyValueError::new_err(value.to_string())
    }
}

impl Display for IncorrectChunkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Expected a {} chunk, got a {} chunk",
            self.expected_chunk_code, self.actual_chunk_code
        )
    }
}

impl Error for IncorrectChunkError {}
impl ChunkError for IncorrectChunkError {}

#[derive(Debug)]
pub enum ChunkLoadError {
    IncorrectChunkError(IncorrectChunkError),
    FieldParseError(FieldParseError),
}

impl From<IncorrectChunkError> for ChunkLoadError {
    fn from(value: IncorrectChunkError) -> Self {
        Self::IncorrectChunkError(value)
    }
}

impl From<FieldParseError> for ChunkLoadError {
    fn from(value: FieldParseError) -> Self {
        Self::FieldParseError(value)
    }
}
