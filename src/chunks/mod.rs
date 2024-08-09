use crate::errors::{
    ChunkError, ChunkParseError, FatalError, FieldParseError, IncorrectChunkError,
};
use ascii::AsciiString;
use bytes::{Buf, Bytes};

pub mod fact;
pub mod fmt;

#[derive(Debug)]
pub(crate) struct Chunk {
    pub id: String,
    pub size: usize,
    pub data: Bytes,
}

impl Chunk {
    fn field_error(&self, field_name: String, reason: String) -> FieldParseError {
        FieldParseError {
            chunk_code: self.id.clone(),
            field_name,
            position: self.size - self.data.remaining(),
            reason,
        }
    }

    pub fn fatal_field_error(&self, field_name: &str, reason: String) -> FatalError {
        FatalError {
            inner: self.field_error(field_name.to_string(), reason).into(),
        }
    }

    pub fn pop_from_data(chunk_data: &mut Bytes) -> Result<Self, ChunkError> {
        if chunk_data.len() < 8 {
            Err(ChunkParseError::new_idless(
                "Invalid chunk: too short".to_string(),
            ))?
        };

        let id = AsciiString::from_ascii(chunk_data.split_to(4))
            .map_err(|err| ChunkParseError::new_idless(format!("Invalid chunk code: {}", err)))?
            .to_string();

        let size = chunk_data.get_u32_le().try_into()?;
        if size > chunk_data.len() {
            Err(ChunkParseError {
                chunk_code: id.clone(),
                reason: "Requested chunk size too large".to_string(),
            })?
        }

        let data = chunk_data.split_to(size);

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

    pub fn load_type(self) -> Result<ChunkType, ChunkError> {
        Ok(match self.id.as_str() {
            "fmt " => ChunkType::Fmt(self.try_into()?),
            "fact" => ChunkType::Fact(self.try_into()?),
            "data" => ChunkType::Data(self),
            _ => ChunkType::Unknown(self),
        })
    }

    fn validate_field_length(&self, len: usize, field_name: &str) -> Result<(), FieldParseError> {
        match self.data.remaining() >= len {
            true => Ok(()),
            false => Err(self.field_error(
                field_name.into(),
                format!(
                    "{} bytes expected, {} left remaining in chunk.",
                    len,
                    self.data.remaining()
                ),
            )),
        }
    }

    pub fn data_bytes<const N: usize>(
        &mut self,
        field_name: &str,
    ) -> Result<[u8; N], FieldParseError> {
        self.validate_field_length(N, field_name)?;
        let popped_chunk = {
            *self
                .data
                .first_chunk::<N>()
                .expect("Chunk length already verified")
        };
        self.data.advance(N);
        Ok(popped_chunk)
    }

    pub fn data_string<const N: usize>(
        &mut self,
        field_name: &str,
    ) -> Result<String, FieldParseError> {
        let chunk_code = self.id.to_owned();
        let position = self.size - self.data.len();

        match AsciiString::from_ascii(self.data_bytes::<N>(field_name)?) {
            Ok(data) => Ok(data.to_string()),
            Err(err) => Err(FieldParseError {
                chunk_code,
                field_name: field_name.to_string(),
                position,
                reason: err.to_string(),
            }),
        }
    }

    pub fn data_u16(&mut self, field_name: &str) -> Result<u16, FieldParseError> {
        self.validate_field_length(2, field_name)?;
        Ok(self.data.get_u16_le())
    }

    pub fn data_u32(&mut self, field_name: &str) -> Result<u32, FieldParseError> {
        self.validate_field_length(4, field_name)?;
        Ok(self.data.get_u32_le())
    }
}

impl Iterator for Chunk {
    type Item = Result<ChunkType, ChunkError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.is_empty() {
            return None;
        }

        let next_chunk: Result<Chunk, ChunkError> =
            Chunk::pop_from_data(&mut self.data).map_err(|_| {
                IncorrectChunkError {
                    expected_chunk_code: "Container chunk".to_string(),
                    actual_chunk_code: "Non-container chunk".to_string(),
                }
                .into()
            });

        Some(next_chunk.map_or_else(Err, |chunk| chunk.load_type()))
    }
}

#[derive(Debug)]
pub enum ChunkType {
    Fmt(fmt::Fmt),
    Fact(fact::Fact),
    Data(Chunk),
    Unknown(Chunk),
}
