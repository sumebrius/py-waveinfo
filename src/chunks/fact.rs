use super::{errors::ChunkLoadError, Chunk};

pub struct Fact {
    pub samples: u32,
}

impl<'a> TryFrom<&'a Chunk<'a>> for Fact {
    type Error = ChunkLoadError;

    fn try_from(chunk: &'a Chunk<'a>) -> Result<Self, Self::Error> {
        chunk.validate_type("fact")?;

        Ok(Self {
            samples: chunk.data_u32(0, "dwSampleLength")?,
        })
    }
}
