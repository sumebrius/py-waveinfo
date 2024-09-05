use num_enum::TryFromPrimitive;
use pyo3::{prelude::*, types::PyDelta};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{formats::Format, util::parse_guid};

use super::wave::WavFile;

#[pyclass(get_all, module = "waveinfo")]
#[derive(Clone, Debug, PartialEq)]
pub struct WavDetail {
    pub format: Format,
    pub duration: f64,
    pub channels: usize,
    pub bit_depth: usize,
    pub sample_rate: usize,
    pub channel_positions: Vec<SpeakerPosition>,
}

#[pymethods]
impl WavDetail {
    #[new]
    fn new(file: super::ConstructorArg) -> PyResult<Self> {
        let wavfile = WavFile::rs_new(file)?;
        Ok((&wavfile.raw_details).into())
    }

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
        let format = match Format::from_tag(value.format_tag) {
            Format::EXTENSIBLE => match value.subformat {
                Some(guid) => {
                    let (fmt_bytes, magic_str) = guid.split_at(2);
                    if magic_str
                        == [
                            0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x80, 0x00, 0x00, 0xAA, 0x00, 0x38,
                            0x9B, 0x71,
                        ]
                    {
                        Format::from_tag(u16::from_le_bytes(
                            fmt_bytes.try_into().expect("Byte length already validated"),
                        ))
                    } else {
                        Format::UNKNOWN
                    }
                }
                None => Format::UNKNOWN,
            },
            other => other,
        };

        WavDetail {
            format,
            duration: value.total_samples as f64 / value.sample_rate as f64,
            channels: value.channels,
            bit_depth: value.sample_depth,
            sample_rate: value.sample_rate,
            channel_positions: SpeakerPosition::from_mask(value.channel_mask, value.channels),
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
    pub channel_mask: Option<u32>,
    pub subformat: Option<[u8; 16]>,
    pub total_samples: usize,
}

#[pymethods]
impl RawDetail {
    #[new]
    fn new(file: super::ConstructorArg) -> PyResult<Self> {
        let wavfile = WavFile::rs_new(file)?;
        Ok(wavfile.raw_details)
    }

    #[getter]
    fn get_subformat(&self) -> PyResult<Option<String>> {
        Ok(self.subformat.map(parse_guid))
    }
}

#[pyclass(eq, eq_int, frozen, get_all, module = "waveinfo")]
#[derive(PartialEq, Clone, Copy, Debug, TryFromPrimitive, EnumIter)]
#[repr(u32)]
#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
pub enum SpeakerPosition {
    FRONT_LEFT = 0x00000001,
    FRONT_RIGHT = 0x00000002,
    FRONT_CENTER = 0x00000004,
    LOW_FREQUENCY = 0x00000008,
    BACK_LEFT = 0x00000010,
    BACK_RIGHT = 0x00000020,
    FRONT_LEFT_OF_CENTER = 0x00000040,
    FRONT_RIGHT_OF_CENTER = 0x00000080,
    BACK_CENTER = 0x00000100,
    SIDE_LEFT = 0x00000200,
    SIDE_RIGHT = 0x00000400,
    TOP_CENTER = 0x00000800,
    TOP_FRONT_LEFT = 0x00001000,
    TOP_FRONT_CENTER = 0x00002000,
    TOP_FRONT_RIGHT = 0x00004000,
    TOP_BACK_LEFT = 0x00008000,
    TOP_BACK_CENTER = 0x00010000,
    TOP_BACK_RIGHT = 0x00020000,
    RESERVED = 0xFFFFFFFF,
}

impl SpeakerPosition {
    pub(crate) fn from_mask(mask: Option<u32>, channels: usize) -> Vec<Self> {
        const ALL_POSITIONS: u32 = 0xFFFFFFFF;
        let mask = match mask.unwrap_or(ALL_POSITIONS) {
            0 => ALL_POSITIONS,
            mask => mask,
        };

        let mut positions: Vec<Self> = Vec::with_capacity(channels);

        for position in Self::iter() {
            if mask & position as u32 != 0 {
                positions.push(position);
                if positions.len() == channels {
                    break;
                }
            }
        }

        for _ in positions.len()..channels {
            positions.push(Self::RESERVED)
        }

        positions
    }
}
