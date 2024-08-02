use ascii::AsciiString;

mod fmt;

pub mod errors;

pub struct Chunk<'a> {
    pub id: String,
    pub size: usize,
    pub data: &'a [u8],
}

impl<'a> Chunk<'a> {
    pub fn from_data(chunk_data: &'a [u8]) -> Result<Self, errors::ChunkParseError> {
        let id_bytes = chunk_data.get(0..4).ok_or(errors::ChunkParseError::new(
            "Invalid chunk code: too short".to_string(),
        ))?;
        let id = AsciiString::from_ascii(id_bytes)
            .map_err(|err| errors::ChunkParseError::new(format!("Invalid chunk code: {}", err)))?
            .to_string();

        let size_bytes = chunk_data
            .get(4..8)
            .ok_or(errors::ChunkParseError::new_with_id(
                id,
                "Invalid size field".to_string(),
            ))?;
        let size = u32::from_le_bytes(size_bytes.try_into().unwrap())
            .try_into()
            .map_err(|_| {
                errors::ChunkParseError::new_with_id(
                    id,
                    "Chunk size too big for architecture".to_string(),
                )
            })?;

        let data = chunk_data
            .get(8..(8 + size))
            .ok_or(errors::ChunkParseError::new_with_id(
                id,
                "Data out of range".to_string(),
            ))?;

        Ok(Chunk { id, size, data })
    }
}
