use bytes::Bytes;
use pyo3::prelude::*;
use std::{fs::read, path::PathBuf};

mod chunks;
mod formats;

use chunks::{errors::FatalError, Chunk, ChunkType};
use formats::Format;

#[pyclass(get_all)]
struct WavFile {
    format: Format,
    channels: usize,
    sample_rate: u32,
    bit_depth: usize,
    avg_bitrate: u32,
    sample_length: usize,
    // data: Chunk,
}

#[derive(FromPyObject)]
enum ConstructorArg {
    Bytes(Vec<u8>),
    Path(PathBuf),
    // TODO - Accept a `read`able file
}

#[pymethods]
impl WavFile {
    #[new]
    fn new(file: ConstructorArg) -> PyResult<Self> {
        let mut bytes: Bytes = match file {
            ConstructorArg::Bytes(bytes) => bytes,
            ConstructorArg::Path(path) => read(path)?,
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
            .ok_or(FatalError::from(chunks::errors::ChunkParseError::new(
                "Missing fmt chunk".to_string(),
            )))?;

        Ok(WavFile {
            format: formats::Format::from_bytes(&fmt_chunk.format_tag),
            channels: fmt_chunk.channels.into(),
            sample_rate: fmt_chunk.samples_per_sec,
            bit_depth: fmt_chunk.bits_per_sample.into(),
            avg_bitrate: fmt_chunk.avg_bytes_per_sec,
            sample_length: 0,
            // bytes,
        })
    }
}

#[pymodule]
fn pywav(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<WavFile>()?;
    Ok(())
}
