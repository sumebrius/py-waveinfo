use super::{
    errors::{ChunkLoadError, FieldParseError},
    Chunk,
};

pub struct Fmt {
    pub format_tag: [u8; 2],
    pub channels: u16,
    pub samples_per_sec: u32,
    pub avg_bytes_per_sec: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
    pub extension_size: Option<u16>,
    pub valid_bits_per_sample: Option<u16>,
    pub channel_mask: Option<u32>,
    pub sub_format: Option<[u8; 16]>,
}

impl<'a> TryFrom<Chunk<'a>> for Fmt {
    type Error = ChunkLoadError;

    fn try_from(chunk: Chunk) -> Result<Self, Self::Error> {
        chunk.validate_type("fmt ")?;

        let extension_size = chunk
            .data
            .get(16..18)
            .map(|bytes| u16::from_le_bytes(bytes.try_into().expect("Less than 2 bytes returned")));

        let (valid_bits_per_sample, channel_mask, sub_format) = match extension_size {
            Some(40) => {
                let guid: [u8; 16] = chunk
                    .data_bytes(24, 16, "SubFormat")?
                    .try_into()
                    .expect("Less than 16 bytes returned");
                (
                    Some(chunk.data_u16(18, "wValidBitsPerSample")?),
                    Some(chunk.data_u32(10, "dwChannelMask")?),
                    Some(guid),
                )
            }
            Some(0) => (None, None, None),
            None => (None, None, None),
            Some(_) => {
                return Err(FieldParseError {
                    chunk_code: chunk.id,
                    field_name: "cbSize".to_string(),
                    position: 16,
                    reason: "Invalid fmt extension size".to_string(),
                }
                .into())
            }
        };

        Ok(Self {
            format_tag: chunk
                .data_bytes(0, 2, "wFormatTag")?
                .try_into()
                .expect("Less than 2 bytes returned"),
            channels: chunk.data_u16(2, "wChannels")?,
            samples_per_sec: chunk.data_u32(4, "dwSamplesPerSec")?,
            avg_bytes_per_sec: chunk.data_u32(8, "dwAvgBytesPerSec")?,
            block_align: chunk.data_u16(12, "wBlockAlign")?,
            bits_per_sample: chunk.data_u16(14, "wBitsPerSample")?,
            extension_size,
            valid_bits_per_sample,
            channel_mask,
            sub_format,
        })
    }
}
