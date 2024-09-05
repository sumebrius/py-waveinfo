use detail::SpeakerPosition;
use pyo3::types::PyDict;

use super::*;

#[test]
fn constructor() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let locals = PyDict::new_bound(py);
        py.run_bound(
            r#"
from io import BytesIO
from pathlib import Path
bytes_arg = b"RIFF"
path_str_arg = "tests/assets/arc_master.wav"
path_obj_arg = Path(path_str_arg)
file_arg = BytesIO(bytes_arg)"#,
            None,
            Some(&locals),
        )
        .unwrap();

        for arg_name in ["bytes_arg", "path_str_arg", "path_obj_arg", "file_arg"] {
            let arg = locals
                .get_item(arg_name)
                .unwrap()
                .unwrap()
                .extract::<ConstructorArg>();
            assert!(
                arg.is_ok(),
                "argument {} not extracted into ConstructorArg",
                arg_name
            );
            let arg_bytes = Bytes::try_from(arg.unwrap());
            assert!(
                arg_bytes.is_ok(),
                "argument {} not extracted into Bytes",
                arg_name
            );
            assert_eq!(arg_bytes.unwrap().slice(0..4), Bytes::from("RIFF"));
        }
    })
}

#[test]
fn detail_duration_getter() {
    let wav_detail = detail::WavDetail {
        format: crate::formats::Format::UNKNOWN,
        duration: 42.6,
        channels: 1,
        bit_depth: 8,
        sample_rate: 44100,
        channel_positions: vec![detail::SpeakerPosition::FRONT_LEFT],
    };

    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let locals = PyDict::new_bound(py);
        locals
            .set_item("detail_obj", Py::new(py, wav_detail).unwrap())
            .unwrap();

        py.run_bound(
            r#"
from datetime import timedelta
res = detail_obj.duration == timedelta(seconds=42.6)"#,
            None,
            Some(&locals),
        )
        .unwrap();

        assert!(locals
            .get_item("res")
            .unwrap()
            .unwrap()
            .extract::<bool>()
            .unwrap())
    })
}

#[test]
fn detail_converter() {
    let raw = detail::RawDetail {
        format_tag: 0x0006,
        channels: 2,
        sample_rate: 44100,
        data_rate: 88100,
        block_size: 2,
        sample_depth: 8,
        channel_mask: None,
        subformat: None,
        total_samples: 441441,
    };
    let expected = detail::WavDetail {
        format: crate::formats::Format::ALAW,
        duration: 10.01,
        channels: 2,
        bit_depth: 8,
        sample_rate: 44100,
        channel_positions: vec![
            detail::SpeakerPosition::FRONT_LEFT,
            detail::SpeakerPosition::FRONT_RIGHT,
        ],
    };
    assert_eq!(expected, (&raw).into());
}

#[test]
fn ext_detail_converter() {
    let raw = detail::RawDetail {
        format_tag: 0xFFFE,
        channels: 2,
        sample_rate: 192000,
        data_rate: 384000,
        block_size: 2,
        sample_depth: 8,
        channel_mask: Some(0),
        subformat: Some([
            0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x80, 0x00, 0x00, 0xaa, 0x00, 0x38,
            0x9b, 0x71,
        ]),
        total_samples: 1921920,
    };
    let expected = detail::WavDetail {
        format: crate::formats::Format::ALAW,
        duration: 10.01,
        channels: 2,
        bit_depth: 8,
        sample_rate: 192000,
        channel_positions: vec![
            detail::SpeakerPosition::FRONT_LEFT,
            detail::SpeakerPosition::FRONT_RIGHT,
        ],
    };
    assert_eq!(expected, (&raw).into());
}

#[test]
fn speaker_mask() {
    assert_eq!(
        SpeakerPosition::from_mask(None, 2),
        vec![
            detail::SpeakerPosition::FRONT_LEFT,
            detail::SpeakerPosition::FRONT_RIGHT,
        ]
    );
    assert_eq!(
        SpeakerPosition::from_mask(Some(0), 2),
        vec![
            detail::SpeakerPosition::FRONT_LEFT,
            detail::SpeakerPosition::FRONT_RIGHT,
        ]
    );
    assert_eq!(
        SpeakerPosition::from_mask(Some(0x00000003), 2),
        vec![
            detail::SpeakerPosition::FRONT_LEFT,
            detail::SpeakerPosition::FRONT_RIGHT,
        ]
    );
    assert_eq!(
        SpeakerPosition::from_mask(Some(0x00000030), 2),
        vec![
            detail::SpeakerPosition::BACK_LEFT,
            detail::SpeakerPosition::BACK_RIGHT,
        ]
    );
    assert_eq!(
        SpeakerPosition::from_mask(Some(0x00000803), 2),
        vec![
            detail::SpeakerPosition::FRONT_LEFT,
            detail::SpeakerPosition::FRONT_RIGHT,
        ]
    );
    assert_eq!(
        SpeakerPosition::from_mask(Some(0x00000001), 2),
        vec![
            detail::SpeakerPosition::FRONT_LEFT,
            detail::SpeakerPosition::RESERVED,
        ]
    );
}
