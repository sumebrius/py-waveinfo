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
