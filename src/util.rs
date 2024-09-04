use pyo3::prelude::*;

pub(crate) fn parse_guid(bytes: [u8; 16]) -> String {
    let chars = bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<String>>();

    [
        chars[0..4].join(""),
        chars[4..6].join(""),
        chars[6..8].join(""),
        chars[8..10].join(""),
        chars[10..].join(""),
    ]
    .join("-")
}

pub(crate) fn read_from_filelike(filelike: Bound<'_, PyAny>) -> PyResult<Vec<u8>> {
    // This should be unnecessary if this ever becomes reality
    // https://github.com/PyO3/pyo3/issues/933

    let read_result = filelike.call_method0("read")?;
    let buffer = read_result.extract::<Vec<u8>>()?;
    filelike.call_method1("seek", (0,))?;
    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use pyo3::{
        exceptions::PyAttributeError,
        types::{PyDict, PyType},
    };

    use super::*;

    #[test]
    fn guid_parser() {
        let uuid_bytes = 193453761000446423301720482639943054353u128.to_be_bytes();
        assert_eq!(
            "9189d6d0-56ec-49d0-b97d-e56c35983411".to_string(),
            parse_guid(uuid_bytes)
        )
    }

    #[test]
    fn read_filelike_ok() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let locals = PyDict::new_bound(py);
            py.run_bound(
                r#"
import io
filelike = io.BytesIO(b'test')"#,
                None,
                Some(&locals),
            )
            .unwrap();
            let filelike = locals.get_item("filelike").unwrap().unwrap();
            let result = read_from_filelike(filelike);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), Vec::from(b"test"));
        })
    }

    #[test]
    fn read_filelike_bad_object() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let filelike = py.eval_bound("{b'test'}", None, None).unwrap();
            let result = read_from_filelike(filelike);
            assert!(result.is_err());
            assert!(result
                .unwrap_err()
                .get_type_bound(py)
                .is(&PyType::new_bound::<PyAttributeError>(py)));
        })
    }
}
