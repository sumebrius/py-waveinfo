use super::{errors::ChunkLoadError, Chunk};

pub struct Fact {
    pub samples: u32,
}

impl<'a> TryFrom<Chunk<'a>> for Fact {
    type Error = ChunkLoadError;

    fn try_from(chunk: Chunk<'a>) -> Result<Self, Self::Error> {
        chunk.validate_type("fact")?;

        Ok(Self {
            samples: chunk.data_u32(0, "dwSampleLength")?,
        })
    }
}
