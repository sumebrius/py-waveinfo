use ascii::AsciiString;
use errors::IncorrectChunkError;

pub mod errors;
pub mod fact;
pub mod fmt;

pub struct Chunk<'a> {
    pub id: String,
    pub size: usize,
    pub data: &'a [u8],
}

impl<'a> Chunk<'a> {
    fn field_error(
        &self,
        field_name: String,
        position: usize,
        reason: String,
    ) -> errors::FieldParseError {
        errors::FieldParseError {
            chunk_code: self.id.clone(),
            field_name,
            position,
            reason,
        }
    }

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
                id.clone(),
                "Invalid size field".to_string(),
            ))?;
        let size = u32::from_le_bytes(size_bytes.try_into().unwrap())
            .try_into()
            .map_err(|_| {
                errors::ChunkParseError::new_with_id(
                    id.clone(),
                    "Chunk size too big for architecture".to_string(),
                )
            })?;

        let data = chunk_data
            .get(8..(8 + size))
            .ok_or(errors::ChunkParseError::new_with_id(
                id.clone(),
                "Data out of range".to_string(),
            ))?;

        Ok(Chunk { id, size, data })
    }

    pub fn validate_type(&self, expected_type: &str) -> Result<(), IncorrectChunkError> {
        if expected_type == self.id {
            Ok(())
        } else {
            Err(IncorrectChunkError {
                expected_chunk_code: expected_type.to_string(),
                actual_chunk_code: self.id.clone(),
            })
        }
    }

    pub fn data_bytes(
        &self,
        offset: usize,
        len: usize,
        field_name: &str,
    ) -> Result<&[u8], errors::FieldParseError> {
        self.data.get(offset..offset + len).ok_or(self.field_error(
            field_name.into(),
            offset,
            "Too short field".to_string(),
        ))
    }

    pub fn data_string(
        &self,
        offset: usize,
        len: usize,
        field_name: &str,
    ) -> Result<String, errors::FieldParseError> {
        let bytes = self.data_bytes(offset, len, field_name)?;
        Ok(AsciiString::from_ascii(bytes)
            .map_err(|err| self.field_error(field_name.into(), offset, err.to_string()))?
            .to_string())
    }

    pub fn data_u16(
        &self,
        offset: usize,
        field_name: &str,
    ) -> Result<u16, errors::FieldParseError> {
        let bytes: [u8; 2] = self
            .data_bytes(offset, 2, field_name)?
            .try_into()
            .expect("Less than 2 bytes returned");
        Ok(u16::from_le_bytes(bytes))
    }

    pub fn data_u32(
        &self,
        offset: usize,
        field_name: &str,
    ) -> Result<u32, errors::FieldParseError> {
        let bytes: [u8; 4] = self
            .data_bytes(offset, 4, field_name)?
            .try_into()
            .expect("Less than 4 bytes returned");
        Ok(u32::from_le_bytes(bytes))
    }
}
