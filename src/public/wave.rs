use std::collections::HashMap;

use bytes::Bytes;
use pyo3::prelude::*;

use crate::{
    chunks::{fact::Fact, Chunk, ChunkType},
    errors::{FatalError, IncorrectChunkError, MissingChunkError},
};

use super::detail::{RawDetail, WavDetail};

#[pyclass(get_all, module = "waveinfo")]
#[derive(Debug, PartialEq)]
pub struct WavFile {
    pub(crate) raw_details: RawDetail,
    pub(crate) info: HashMap<String, String>,
    //TODO - add this when we do something with it, otherwise it just takes up memory.
    // data: Bytes,
}

#[pymethods]
impl WavFile {
    #[new]
    fn new(file: super::ConstructorArg) -> PyResult<Self> {
        Self::rs_new(file)
    }

    #[getter]
    fn detail(&self) -> WavDetail {
        WavDetail::from(&self.raw_details)
    }
}

impl WavFile {
    pub(super) fn rs_new(file: super::ConstructorArg) -> PyResult<Self> {
        let mut bytes: Bytes = file.try_into()?;

        let mut riff_chunk = Chunk::pop_from_data(&mut bytes).map_err(FatalError::from)?;

        riff_chunk.validate_type("RIFF").map_err(FatalError::from)?;

        if riff_chunk
            .data_string::<4>("WAVEID")
            .map_err(FatalError::from)?
            != "WAVE"
        {
            Err(riff_chunk.fatal_field_error("WAVEID", "Incorrect RIFF type".to_string()))?
        };

        let mut riff_chunks = riff_chunk.typed_iter();

        let fmt_chunk = riff_chunks
            .next_ok()
            .and_then(|chunktype| match chunktype {
                ChunkType::Fmt(chunk) => Some(chunk),
                _ => None,
            })
            .ok_or(FatalError::from(MissingChunkError::new("fmt")))?;

        let mut fact_chunk: Option<Fact> = None;
        let mut info = HashMap::<String, String>::new();

        let data_chunk = loop {
            match riff_chunks.next() {
                Some(result) => {
                    if let Ok(chunktype) = result {
                        match chunktype {
                            ChunkType::Data(chunk) => break chunk,
                            ChunkType::Fmt(_) => Err(FatalError::from(IncorrectChunkError {
                                expected_chunk_code: "Non-fmt".to_string(),
                                actual_chunk_code: "fmt".to_string(),
                            }))?,
                            ChunkType::Fact(chunk) => fact_chunk = Some(chunk),
                            ChunkType::List(chunk) => {
                                let hm: Result<HashMap<String, String>, _> = chunk.try_into();
                                if let Ok(hm) = hm {
                                    info = hm;
                                }
                            }
                            //TODO - handle optional metadata chunks that may appear before data chunk
                            ChunkType::Unknown(_) => (),
                        }
                    }
                }
                None => Err(FatalError::from(MissingChunkError::new("data")))?,
            }
        };

        let sample_length: usize = match fact_chunk {
            Some(chunk) => chunk.samples.try_into()?,
            None => {
                (8 * data_chunk.size)
                    / (fmt_chunk.bits_per_sample as usize * fmt_chunk.channels as usize)
            }
        };

        let sample_depth = match fmt_chunk.valid_bits_per_sample {
            Some(bps) => usize::from(bps),
            None => usize::from(fmt_chunk.bits_per_sample),
        };

        let raw_details = RawDetail {
            format_tag: u16::from_le_bytes(fmt_chunk.format_tag),
            channels: fmt_chunk.channels.into(),
            sample_rate: fmt_chunk.samples_per_sec.try_into()?,
            data_rate: fmt_chunk.avg_bytes_per_sec.try_into()?,
            block_size: fmt_chunk.block_align.into(),
            sample_depth,
            channel_mask: fmt_chunk.channel_mask,
            subformat: fmt_chunk.sub_format,
            total_samples: sample_length,
        };

        Ok(WavFile { raw_details, info })
    }
}
