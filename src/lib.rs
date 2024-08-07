use pyo3::prelude::*;

mod chunks;
mod formats;
mod public;

use public::*;

#[pymodule]
fn pywav(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<wave::WavFile>()?;
    Ok(())
}
