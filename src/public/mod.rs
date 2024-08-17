use bytes::Bytes;
use pyo3::prelude::*;
use std::{fs::read, path::PathBuf};

pub mod detail;
pub mod exceptions;
pub mod wave;

pub use crate::formats::Format;
use crate::util::read_from_filelike;

#[derive(FromPyObject)]
enum ConstructorArg<'py> {
    Bytes(Vec<u8>),
    Path(PathBuf),
    File(Bound<'py, PyAny>),
}

impl<'py> TryFrom<ConstructorArg<'py>> for Bytes {
    type Error = PyErr;

    fn try_from(value: ConstructorArg<'py>) -> Result<Self, Self::Error> {
        Ok(match value {
            ConstructorArg::Bytes(bytes) => bytes,
            ConstructorArg::Path(path) => read(path)?,
            ConstructorArg::File(filelike) => read_from_filelike(filelike)?,
        }
        .into())
    }
}

#[cfg(test)]
mod tests;
