use bytes::Bytes;
use pyo3::prelude::*;
use std::fs::read;

use crate::chunks::{
    errors::{ChunkParseError, FatalError, IncorrectChunkError},
    Chunk, ChunkType,
};
use crate::formats::Format;

#[pyclass(get_all)]
pub struct WavFile {
    format: Format,
    channels: usize,
    sample_rate: u32,
    bit_depth: usize,
    avg_bitrate: u32,
    sample_length: usize,
    // data: Chunk,
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
            .ok_or(FatalError::from(ChunkParseError::new(
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
                    .ok_or(FatalError::from(ChunkParseError::new(
                        "Missing fmt chunk".to_string(),
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
                .try_into()
                .map_err(|_| {
                    ChunkParseError::new_with_id(
                        "data".to_string(),
                        "Requested chunk size too big for architecture".to_string(),
                    )
                })?
        } else {
            8 * data_chunk.size / fmt_chunk.bits_per_sample as usize
        };

        Ok(WavFile {
            format: Format::from_bytes(&fmt_chunk.format_tag),
            channels: fmt_chunk.channels.into(),
            sample_rate: fmt_chunk.samples_per_sec,
            bit_depth: fmt_chunk.bits_per_sample.into(),
            avg_bitrate: fmt_chunk.avg_bytes_per_sec,
            sample_length,
            // bytes,
        })
    }
}
