use ascii::AsciiString;
use bytes::{Buf, Bytes};
use errors::{ChunkLoadError, IncorrectChunkError};

pub mod errors;
pub mod fact;
pub mod fmt;

pub struct Chunk {
    pub id: String,
    pub size: usize,
    pub data: Bytes,
}

impl Chunk {
    fn field_error(&self, field_name: String, reason: String) -> errors::FieldParseError {
        errors::FieldParseError {
            chunk_code: self.id.clone(),
            field_name,
            position: self.size - self.data.remaining(),
            reason,
        }
    }

    pub fn fatal_field_error(&self, field_name: &str, reason: String) -> errors::FatalError {
        errors::FatalError {
            inner: self.field_error(field_name.to_string(), reason).into(),
        }
    }

    pub fn pop_from_data(chunk_data: &mut Bytes) -> Result<Self, errors::ChunkParseError> {
        if chunk_data.len() < 8 {
            return Err(errors::ChunkParseError::new(
                "Invalid chunk: too short".to_string(),
            ));
        };

        let id = AsciiString::from_ascii(chunk_data.split_to(4))
            .map_err(|err| errors::ChunkParseError::new(format!("Invalid chunk code: {}", err)))?
            .to_string();

        let size = chunk_data.get_u32_le().try_into().map_err(|_| {
            errors::ChunkParseError::new_with_id(
                id.clone(),
                "Requested chunk size too big for architecture".to_string(),
            )
        })?;
        if size > chunk_data.len() {
            return Err(errors::ChunkParseError {
                chunk_code: id.clone(),
                reason: "Requested chunk size too large".to_string(),
            });
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

    pub fn load_type(self) -> Result<ChunkType, ChunkLoadError> {
        Ok(match self.id.as_str() {
            "fmt " => ChunkType::Fmt(self.try_into()?),
            "fact" => ChunkType::Fact(self.try_into()?),
            "data" => ChunkType::Data(self),
            _ => ChunkType::Unknown(self),
        })
    }

    fn validate_field_length(
        &self,
        len: usize,
        field_name: &str,
    ) -> Result<(), errors::FieldParseError> {
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
    ) -> Result<[u8; N], errors::FieldParseError> {
        self.validate_field_length(N, field_name)?;
        Ok(*self
            .data
            .first_chunk::<N>()
            .expect("Chunk length already verified"))
    }

    pub fn data_string<const N: usize>(
        &mut self,
        field_name: &str,
    ) -> Result<String, errors::FieldParseError> {
        let chunk_code = self.id.to_owned();
        let position = self.size - self.data.len();

        match AsciiString::from_ascii(self.data_bytes::<N>(field_name)?) {
            Ok(data) => Ok(data.to_string()),
            Err(err) => Err(errors::FieldParseError {
                chunk_code,
                field_name: field_name.to_string(),
                position,
                reason: err.to_string(),
            }),
        }
    }

    pub fn data_u16(&mut self, field_name: &str) -> Result<u16, errors::FieldParseError> {
        self.validate_field_length(2, field_name)?;
        Ok(self.data.get_u16_le())
    }

    pub fn data_u32(&mut self, field_name: &str) -> Result<u32, errors::FieldParseError> {
        self.validate_field_length(4, field_name)?;
        Ok(self.data.get_u32_le())
    }
}

pub enum ChunkType {
    Fmt(fmt::Fmt),
    Fact(fact::Fact),
    Data(Chunk),
    Unknown(Chunk),
}

impl Iterator for Chunk {
    type Item = Result<ChunkType, ChunkLoadError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.is_empty() {
            return None;
        }

        let next_chunk: Result<Chunk, ChunkLoadError> = Chunk::pop_from_data(&mut self.data)
            .map_err(|_| {
                IncorrectChunkError {
                    expected_chunk_code: "Container chunk".to_string(),
                    actual_chunk_code: "Non-container chunk".to_string(),
                }
                .into()
            });

        Some(next_chunk.map_or_else(Err, |chunk| chunk.load_type()))
    }
}
