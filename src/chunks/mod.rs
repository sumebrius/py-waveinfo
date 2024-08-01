use ascii::AsciiString;
use pyo3::{exceptions::PyValueError, PyErr};
use std::{error::Error, fmt::Display};

mod fmt;

#[derive(Debug)]
pub struct ChunkParseError {
    chunk_code: String,
    reason: String,
}

impl From<ChunkParseError> for PyErr {
    fn from(value: ChunkParseError) -> Self {
        PyValueError::new_err(value.reason)
    }
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

pub struct Chunk<'a> {
    pub id: String,
    pub size: usize,
    pub data: &'a [u8],
}

impl<'a> Chunk<'a> {
    pub fn from_data(chunk_data: &'a [u8]) -> Result<Self, ChunkParseError> {
        let id_bytes = chunk_data.get(0..4).ok_or(ChunkParseError::new(
            None,
            "Invalid chunk code: too short".to_string(),
        ))?;
        let id = AsciiString::from_ascii(id_bytes)
            .map_err(|err| ChunkParseError::new(None, format!("Invalid chunk code: {}", err)))?
            .to_string();

        let size_bytes = chunk_data.get(4..8).ok_or(ChunkParseError::new(
            Some(&id),
            "Invalid size field".to_string(),
        ))?;
        let size = u32::from_le_bytes(size_bytes.try_into().unwrap())
            .try_into()
            .map_err(|_| {
                ChunkParseError::new(Some(&id), "Chunk size too big for architecture".to_string())
            })?;

        let data = chunk_data.get(8..(8 + size)).ok_or(ChunkParseError::new(
            Some(&id),
            "Data out of range".to_string(),
        ))?;

        Ok(Chunk { id, size, data })
    }
}
