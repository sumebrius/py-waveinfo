use pyo3::{prelude::*, types::PyDelta};

use crate::formats::Format;

#[pyclass(get_all)]
#[derive(Clone)]
pub struct WavDetail {
    pub format: Format,
    // TODO - make a PyDuration
    pub duration: usize,
    pub channels: usize,
    pub bit_depth: usize,
    pub sample_rate: usize,
}

#[pyclass(get_all)]
#[derive(Clone)]
pub struct RawDetail {
    pub format: Format,
    pub channels: usize,
    pub sample_rate: usize,
    pub data_rate: usize,
    pub block_size: usize,
    pub sample_depth: usize,
    // TODO - turn this into enum and add to WavDetail too
    pub channel_mask: Option<u32>,
    pub subformat: Option<String>,
    pub total_samples: usize,
}
