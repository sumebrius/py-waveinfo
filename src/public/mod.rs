use pyo3::prelude::*;
use std::path::PathBuf;

pub mod wave;

#[derive(FromPyObject)]
enum ConstructorArg {
    Bytes(Vec<u8>),
    Path(PathBuf),
    // TODO - Accept a `read`able file
}
