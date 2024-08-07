use pyo3::prelude::*;

mod chunks;
mod formats;
mod public;
mod util;

use public::*;

#[pymodule]
fn pywav(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<wave::WavFile>()?;
    m.add_class::<detail::WavDetail>()?;
    m.add_class::<detail::RawDetail>()?;
    Ok(())
}
