use pyo3::prelude::*;

#[pyclass(eq, eq_int, frozen, get_all)]
#[derive(PartialEq, Clone, Copy)]
pub enum Format {
    Pcm,
    IeeeFloat,
    Alaw,
    Mulaw,
    Extensible,
    Unknown,
}

impl Format {
    pub fn from_bytes(bytes: &[u8; 2]) -> Self {
        match u16::from_ne_bytes(*bytes) {
            0x0001 => Self::Pcm,
            0x0003 => Self::IeeeFloat,
            0x0006 => Self::Alaw,
            0x0007 => Self::Mulaw,
            0xFFFE => Self::Extensible,
            _ => Self::Unknown,
        }
    }

    pub fn requires_fact_chunk(&self) -> bool {
        !matches!(self, Format::Pcm)
    }
}
