use super::Chunk;
use crate::errors::ChunkError;

#[derive(Debug, PartialEq)]
pub(crate) struct Fact {
    pub samples: u32,
}

impl TryFrom<Chunk> for Fact {
    type Error = ChunkError;

    fn try_from(mut chunk: Chunk) -> Result<Self, Self::Error> {
        chunk.validate_type("fact")?;

        Ok(Self {
            samples: chunk.data_u32("dwSampleLength")?,
        })
    }
}
