use pyo3::prelude::*;

mod chunks;
mod errors;
mod formats;
mod public;
mod util;

use public::*;

#[pymodule]
fn waveinfo(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<wave::WavFile>()?;
    m.add_class::<detail::WavDetail>()?;
    m.add_class::<detail::RawDetail>()?;
    m.add_class::<Format>()?;
    m.add(
        "WavLoadError",
        py.get_type_bound::<crate::public::exceptions::WavLoadError>(),
    )?;
    Ok(())
}
