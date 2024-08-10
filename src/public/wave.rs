use bytes::Bytes;
use pyo3::prelude::*;
use std::fs::read;

use crate::{
    chunks::{Chunk, ChunkType},
    errors::{ChunkParseError, FatalError, IncorrectChunkError},
    formats::Format,
    util::parse_guid,
};

use super::detail::{RawDetail, WavDetail};

#[pyclass(get_all, module = "waveinfo")]
pub struct WavFile {
    detail: WavDetail,
    raw_details: RawDetail,
    //TODO - add this when we do something with it, otherwise it just takes up memory.
    // data: Bytes,
}

#[pymethods]
impl WavFile {
    #[new]
    fn new(file: super::ConstructorArg) -> PyResult<Self> {
        let mut bytes: Bytes = match file {
            super::ConstructorArg::Bytes(bytes) => bytes,
            super::ConstructorArg::Path(path) => read(path)?,
        }
        .into();

        let mut riff_chunk = Chunk::pop_from_data(&mut bytes).map_err(FatalError::from)?;

        riff_chunk.validate_type("RIFF").map_err(FatalError::from)?;

        if riff_chunk
            .data_string::<4>("WAVEID")
            .map_err(FatalError::from)?
            != "WAVE"
        {
            Err(riff_chunk.fatal_field_error("WAVEID", "Incorrect RIFF type".to_string()))?
        };

        let fmt_chunk = riff_chunk
            .next()
            .transpose()
            .ok()
            .flatten()
            .and_then(|chunktype| match chunktype {
                ChunkType::Fmt(chunk) => Some(chunk),
                _ => None,
            })
            .ok_or(FatalError::from(ChunkParseError::new_idless(
                "Missing fmt chunk".to_string(),
            )))?;

        let file_format = Format::from_bytes(&fmt_chunk.format_tag);

        let fact_chunk = if file_format.requires_fact_chunk() {
            Some(
                riff_chunk
                    .next()
                    .transpose()
                    .ok()
                    .flatten()
                    .and_then(|chunktype| match chunktype {
                        ChunkType::Fact(chunk) => Some(chunk),
                        _ => None,
                    })
                    .ok_or(FatalError::from(ChunkParseError::new_idless(
                        "Missing fact chunk".to_string(),
                    )))?,
            )
        } else {
            None
        };

        let data_chunk = loop {
            match riff_chunk.next() {
                Some(result) => {
                    if let Ok(chunktype) = result {
                        match chunktype {
                            ChunkType::Data(chunk) => break chunk,
                            //TODO - handle optional metadata chunks that may appear before data chunk
                            ChunkType::Unknown(_) => (),
                            ChunkType::Fmt(_) => Err(FatalError::from(IncorrectChunkError {
                                expected_chunk_code: "Any metadata".to_string(),
                                actual_chunk_code: "fmt".to_string(),
                            }))?,
                            ChunkType::Fact(_) => Err(FatalError::from(IncorrectChunkError {
                                expected_chunk_code: "Any metadata".to_string(),
                                actual_chunk_code: "fact".to_string(),
                            }))?,
                        }
                    }
                }
                None => Err(FatalError::from(IncorrectChunkError {
                    expected_chunk_code: "data".to_string(),
                    actual_chunk_code: "".to_string(),
                }))?,
            }
        };

        let sample_length: usize = if file_format.requires_fact_chunk() {
            fact_chunk
                .expect("Fact chunk requirement already verified")
                .samples
                .try_into()?
        } else {
            8 * data_chunk.size / fmt_chunk.bits_per_sample as usize
        };

        let sample_depth = match fmt_chunk.valid_bits_per_sample {
            Some(bps) => usize::from(bps),
            None => usize::from(fmt_chunk.bits_per_sample),
        };

        let raw_details = RawDetail {
            format: Format::from_bytes(&fmt_chunk.format_tag),
            channels: fmt_chunk.channels.into(),
            sample_rate: fmt_chunk.samples_per_sec.try_into()?,
            data_rate: fmt_chunk.avg_bytes_per_sec.try_into()?,
            block_size: fmt_chunk.block_align.into(),
            sample_depth,
            channel_mask: fmt_chunk.channel_mask,
            subformat: fmt_chunk.sub_format.map(parse_guid),
            total_samples: sample_length,
        };

        let detail = WavDetail {
            format: raw_details.format,
            duration: raw_details.total_samples as f64 / raw_details.sample_rate as f64,
            channels: raw_details.channels,
            bit_depth: raw_details.sample_depth,
            sample_rate: raw_details.sample_rate,
        };

        Ok(WavFile {
            detail,
            raw_details,
        })
    }
}
