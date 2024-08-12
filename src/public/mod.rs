use pyo3::prelude::*;
use std::path::PathBuf;

pub mod detail;
pub mod exceptions;
pub mod wave;

pub use crate::formats::Format;

#[derive(FromPyObject)]
enum ConstructorArg<'py> {
    Bytes(Vec<u8>),
    Path(PathBuf),
    File(Bound<'py, PyAny>),
    // TODO - Accept a `read`able file
}
