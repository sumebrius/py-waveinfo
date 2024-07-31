use pyo3::prelude::*;
use std::fs::read;
use std::path::PathBuf;

mod chunks;

#[pyclass]
struct WavFile {
    #[pyo3(get)]
    bytes: Vec<u8>,
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
        let bytes: Vec<u8> = match file {
            ConstructorArg::Bytes(bytes) => bytes,
            ConstructorArg::Path(path) => read(path)?,
        };
        Ok(WavFile { bytes })
    }
}

#[pymodule]
fn pywav(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<WavFile>()?;
    Ok(())
}
