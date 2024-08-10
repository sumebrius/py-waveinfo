use num_enum::TryFromPrimitive;
use pyo3::prelude::*;

#[pyclass(
    eq,
    eq_int,
    frozen,
    get_all,
    rename_all = "SCREAMING_SNAKE_CASE",
    module = "waveinfo"
)]
#[derive(PartialEq, Clone, Copy, TryFromPrimitive)]
#[repr(u16)]
pub enum Format {
    Pcm = 0x0001,
    IeeeFloat = 0x0003,
    Alaw = 0x0006,
    Mulaw = 0x0007,
    Extensible = 0xFFFE,
    Unknown,
}

impl Format {
    pub fn from_bytes(bytes: &[u8; 2]) -> Self {
        Self::try_from(u16::from_ne_bytes(*bytes)).unwrap_or(Self::Unknown)
    }

    pub fn requires_fact_chunk(&self) -> bool {
        !matches!(self, Format::Pcm)
    }
}
