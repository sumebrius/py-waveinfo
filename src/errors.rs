use pyo3::PyErr;
use std::error::Error;
use std::fmt::{Debug, Display};
use std::num::TryFromIntError;

use crate::exceptions::WavLoadError;

#[derive(Debug, PartialEq)]
pub(crate) struct ChunkParseError {
    pub chunk_code: String,
    pub reason: String,
}

impl ChunkParseError {
    pub fn new_idless(reason: String) -> Self {
        Self {
            chunk_code: "Unknown".to_string(),
            reason,
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct FieldParseError {
    pub chunk_code: String,
    pub field_name: String,
    pub position: usize,
    pub reason: String,
}

#[derive(Debug, PartialEq)]
pub(crate) struct IncorrectChunkError {
    pub expected_chunk_code: String,
    pub actual_chunk_code: String,
}

#[derive(Debug, PartialEq)]
pub(crate) struct MissingChunkError {
    pub expected_chunk_code: String,
}

impl MissingChunkError {
    pub fn new(chunk: &str) -> Self {
        Self {
            expected_chunk_code: chunk.to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum ChunkError {
    ChunkParse(ChunkParseError),
    IncorrectChunk(IncorrectChunkError),
    MissingChunk(MissingChunkError),
    FieldParse(FieldParseError),
    TryFromInt(TryFromIntError),
}

impl Display for ChunkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChunkError::ChunkParse(err) => {
                write!(
                    f,
                    "Unable to parse {} chunk: {}",
                    err.chunk_code, err.reason
                )
            }
            ChunkError::IncorrectChunk(err) => {
                write!(
                    f,
                    "Expected a {} chunk, got a {} chunk",
                    err.expected_chunk_code, err.actual_chunk_code
                )
            }
            ChunkError::MissingChunk(err) => {
                write!(f, "No {} chunk found", err.expected_chunk_code)
            }
            ChunkError::FieldParse(err) => {
                write!(
                    f,
                    "Unable to parse {} chunk field {} byte {}: {}",
                    err.chunk_code, err.field_name, err.position, err.reason
                )
            }
            ChunkError::TryFromInt(err) => Debug::fmt(&err, f),
        }
    }
}

impl From<ChunkParseError> for ChunkError {
    fn from(value: ChunkParseError) -> Self {
        Self::ChunkParse(value)
    }
}

impl From<IncorrectChunkError> for ChunkError {
    fn from(value: IncorrectChunkError) -> Self {
        Self::IncorrectChunk(value)
    }
}

impl From<MissingChunkError> for ChunkError {
    fn from(value: MissingChunkError) -> Self {
        Self::MissingChunk(value)
    }
}

impl From<FieldParseError> for ChunkError {
    fn from(value: FieldParseError) -> Self {
        Self::FieldParse(value)
    }
}

impl From<TryFromIntError> for ChunkError {
    fn from(value: TryFromIntError) -> Self {
        Self::TryFromInt(value)
    }
}

impl Error for ChunkError {}

#[derive(Debug)]
pub(crate) struct FatalError {
    pub inner: ChunkError,
}

impl Display for FatalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl From<FatalError> for PyErr {
    fn from(value: FatalError) -> Self {
        WavLoadError::new_err(value.to_string())
    }
}

impl From<ChunkError> for FatalError {
    fn from(value: ChunkError) -> Self {
        Self { inner: value }
    }
}

impl From<ChunkParseError> for FatalError {
    fn from(value: ChunkParseError) -> Self {
        Self {
            inner: value.into(),
        }
    }
}

impl From<IncorrectChunkError> for FatalError {
    fn from(value: IncorrectChunkError) -> Self {
        Self {
            inner: value.into(),
        }
    }
}

impl From<MissingChunkError> for FatalError {
    fn from(value: MissingChunkError) -> Self {
        Self {
            inner: value.into(),
        }
    }
}

impl From<FieldParseError> for FatalError {
    fn from(value: FieldParseError) -> Self {
        Self {
            inner: value.into(),
        }
    }
}

impl Error for FatalError {}
