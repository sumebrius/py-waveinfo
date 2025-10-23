use pyo3::prelude::*;

mod chunks;
mod errors;
mod formats;
mod public;
mod util;

#[pymodule]
mod waveinfo {
    use crate::public::*;

    #[pymodule_export]
    use crate::formats::Format;
    #[pymodule_export]
    use detail::RawDetail;
    #[pymodule_export]
    use detail::WavDetail;
    #[pymodule_export]
    use exceptions::WavLoadError;
    #[pymodule_export]
    use wave::WavFile;
}
