use super::Chunk;
use crate::errors::ChunkError;

#[derive(Debug, PartialEq)]
pub(crate) struct Fmt {
    pub format_tag: [u8; 2],
    pub channels: u16,
    pub samples_per_sec: u32,
    pub avg_bytes_per_sec: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
    #[allow(dead_code)] // Part of the spec
    pub extension_size: Option<u16>,
    pub valid_bits_per_sample: Option<u16>,
    pub channel_mask: Option<u32>,
    pub sub_format: Option<[u8; 16]>,
}

impl TryFrom<Chunk> for Fmt {
    type Error = ChunkError;

    fn try_from(mut chunk: Chunk) -> Result<Self, Self::Error> {
        chunk.validate_type("fmt ")?;

        let format_tag = chunk.data_bytes::<2>("wFormatTag")?;
        let channels = chunk.data_u16("wChannels")?;
        let samples_per_sec = chunk.data_u32("dwSamplesPerSec")?;
        let avg_bytes_per_sec = chunk.data_u32("dwAvgBytesPerSec")?;
        let block_align = chunk.data_u16("wBlockAlign")?;
        let bits_per_sample = chunk.data_u16("wBitsPerSample")?;

        let extension_size = match chunk.data.len() {
            0 => None,
            _ => Some(chunk.data_u16("cbSize")?),
        };

        let (valid_bits_per_sample, channel_mask, sub_format) =
            if let Some(extension_size) = extension_size {
                if extension_size as usize != chunk.data.len() {
                    return Err(chunk
                        .field_error(
                            "cbSize".to_string(),
                            format!(
                                "Extension size mismatch. Reported: {}. Found: {}",
                                extension_size,
                                chunk.data.len()
                            ),
                        )
                        .into());
                }
                match extension_size {
                    0 => (None, None, None),
                    22 => (
                        Some(chunk.data_u16("wValidBitsPerSample")?),
                        Some(chunk.data_u32("dwChannelMask")?),
                        Some(chunk.data_bytes::<16>("SubFormat")?),
                    ),
                    other => Err(chunk.field_error(
                        "cbSize".to_string(),
                        format!("Invalid fmt extension size: {}", other),
                    ))?,
                }
            } else {
                (None, None, None)
            };

        Ok(Self {
            format_tag,
            channels,
            samples_per_sec,
            avg_bytes_per_sec,
            block_align,
            bits_per_sample,
            extension_size,
            valid_bits_per_sample,
            channel_mask,
            sub_format,
        })
    }
}
