use pyo3::{prelude::*, types::PyDelta};

use crate::formats::Format;

#[pyclass(get_all, module = "waveinfo")]
#[derive(Clone, Debug, PartialEq)]
pub struct WavDetail {
    pub format: Format,
    pub duration: f64,
    pub channels: usize,
    pub bit_depth: usize,
    pub sample_rate: usize,
}

#[pymethods]
impl WavDetail {
    #[getter]
    fn get_duration<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDelta>> {
        let days = 0;
        let seconds = self.duration.trunc() as i32;
        let microseconds = (self.duration.fract() * 1_000_000.0) as i32;
        PyDelta::new_bound(py, days, seconds, microseconds, true)
    }
}

impl From<&RawDetail> for WavDetail {
    fn from(value: &RawDetail) -> Self {
        WavDetail {
            format: Format::from_tag(value.format_tag),
            duration: value.total_samples as f64 / value.sample_rate as f64,
            channels: value.channels,
            bit_depth: value.sample_depth,
            sample_rate: value.sample_rate,
        }
    }
}

#[pyclass(get_all, module = "waveinfo")]
#[derive(Clone)]
pub struct RawDetail {
    pub format_tag: u16,
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
