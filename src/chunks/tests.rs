use std::collections::HashMap;

use bytes::Bytes;

use super::*;

#[test]
fn pop_from_data() {
    let mut good_chunk = Bytes::from_static(&[
        0x54, 0x45, 0x53, 0x54, 0x04, 0x00, 0x00, 0x00, 0x44, 0x41, 0x54, 0x41, 0x44, 0x41, 0x54,
    ]);
    let mut null_chunk = Bytes::from_static(&[0x54, 0x45, 0x53]);
    let mut short_chunk =
        Bytes::from_static(&[0x54, 0x45, 0x53, 0x54, 0x04, 0x00, 0x00, 0x00, 0x44, 0x41]);
    let mut long_chunk = Bytes::from_static(&[
        0x54, 0x45, 0x53, 0x54, 0x08, 0x00, 0x00, 0x00, 0x44, 0x41, 0x54, 0x41,
    ]);

    assert!(Chunk::pop_from_data(&mut null_chunk).is_err());
    assert!(Chunk::pop_from_data(&mut short_chunk).is_err());
    assert!(Chunk::pop_from_data(&mut long_chunk).is_err());

    let chunk_res = Chunk::pop_from_data(&mut good_chunk);
    assert!(chunk_res.is_ok());
    let chunk = chunk_res.unwrap();
    assert_eq!(chunk.id, "TEST");
    assert_eq!(chunk.size, 4);
    assert_eq!(chunk.data, Bytes::from_static(&[0x44, 0x41, 0x54, 0x41]))
}

#[test]
fn pop_with_padding_byte() {
    let mut chunk_data = Bytes::from_static(&[
        0x54, 0x45, 0x53, 0x54, 0x03, 0x00, 0x00, 0x00, 0x44, 0x41, 0x54, 0x00, 0x42, 0x41, 0x54,
    ]);
    let chunk = Chunk::pop_from_data(&mut chunk_data).unwrap();
    assert_eq!(chunk.size, 3);
    assert_eq!(chunk.data, Bytes::from_static(&[0x44, 0x41, 0x54]));
    assert_eq!(chunk_data, Bytes::from_static(&[0x42, 0x41, 0x54]));
}

#[test]
fn data_bytes() {
    let mut chunk = Chunk {
        id: "".to_string(),
        size: 8,
        data: Bytes::from_static(b"TESTDATA"),
    };

    assert_eq!(chunk.data_bytes::<4>("").unwrap(), *b"TEST");
    assert_eq!(chunk.data, Bytes::from_static(b"DATA"));
    assert!(chunk.data_bytes::<8>("").is_err());
}

#[test]
fn pop_zstring() {
    let mut chunk = Chunk {
        id: "".to_string(),
        size: 8,
        data: Bytes::from_static(&[0x54, 0x45, 0x53, 0x54, 0x00, 0x44, 0x41, 0x54, 0x41]),
    };

    assert_eq!(chunk.data_zstring("test").unwrap(), "TEST".to_string());
    assert_eq!(chunk.data, Bytes::from_static(&[0x44, 0x41, 0x54, 0x41]));
    assert!(chunk.data_zstring("test").is_err());
}

#[test]
fn to_fact_chunk() {
    let mut chunk_data = Bytes::from_static(&[
        0x66, 0x61, 0x63, 0x74, 0x04, 0x00, 0x00, 0x00, 0x21, 0x96, 0x00, 0x00,
    ]);
    let chunk = Chunk::pop_from_data(&mut chunk_data)
        .unwrap()
        .load_type()
        .unwrap();
    let expected_chunk = ChunkType::Fact(fact::Fact { samples: 0x9621 });
    assert_eq!(chunk, expected_chunk);
}

#[test]
fn to_std_fmt_chunk() {
    let mut chunk_data = Bytes::from_static(&[
        0x66, 0x6d, 0x74, 0x20, 0x10, 0x00, 0x00, 0x00, 0x01, 0x00, 0x02, 0x00, 0x00, 0xee, 0x02,
        0x00, 0x00, 0xb8, 0x0b, 0x00, 0x04, 0x00, 0x10, 0x00,
    ]);
    let chunk = Chunk::pop_from_data(&mut chunk_data)
        .unwrap()
        .load_type()
        .unwrap();
    let expected_chunk = ChunkType::Fmt(fmt::Fmt {
        format_tag: [0x01, 0x00],
        channels: 2,
        samples_per_sec: 192000,
        avg_bytes_per_sec: 768000,
        block_align: 4,
        bits_per_sample: 16,
        extension_size: None,
        valid_bits_per_sample: None,
        channel_mask: None,
        sub_format: None,
    });
    assert_eq!(chunk, expected_chunk);
}

#[test]
fn to_ex_fmt_chunk() {
    let mut chunk_data = Bytes::from_static(&[
        0x66, 0x6d, 0x74, 0x20, 0x12, 0x00, 0x00, 0x00, 0x06, 0x00, 0x02, 0x00, 0x44, 0xac, 0x00,
        0x00, 0x88, 0x58, 0x01, 0x00, 0x02, 0x00, 0x08, 0x00, 0x00, 0x00,
    ]);
    let chunk = Chunk::pop_from_data(&mut chunk_data)
        .unwrap()
        .load_type()
        .unwrap();
    let expected_chunk = ChunkType::Fmt(fmt::Fmt {
        format_tag: [0x06, 0x00],
        channels: 2,
        samples_per_sec: 44100,
        avg_bytes_per_sec: 88200,
        block_align: 2,
        bits_per_sample: 8,
        extension_size: Some(0),
        valid_bits_per_sample: None,
        channel_mask: None,
        sub_format: None,
    });
    assert_eq!(chunk, expected_chunk);
}

#[test]
fn to_extensible_fmt_chunk() {
    let mut chunk_data = Bytes::from_static(&[
        0x66, 0x6d, 0x74, 0x20, 0x28, 0x00, 0x00, 0x00, 0xfe, 0xff, 0x02, 0x00, 0x00, 0xee, 0x02,
        0x00, 0x00, 0xdc, 0x05, 0x00, 0x02, 0x00, 0x08, 0x00, 0x16, 0x00, 0x08, 0x00, 0x03, 0x00,
        0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x80, 0x00, 0x00, 0xaa, 0x00,
        0x38, 0x9b, 0x71,
    ]);
    let chunk = Chunk::pop_from_data(&mut chunk_data)
        .unwrap()
        .load_type()
        .unwrap();
    let expected_chunk = ChunkType::Fmt(fmt::Fmt {
        format_tag: [0xFE, 0xFF],
        channels: 2,
        samples_per_sec: 192000,
        avg_bytes_per_sec: 384000,
        block_align: 2,
        bits_per_sample: 8,
        extension_size: Some(22),
        valid_bits_per_sample: Some(8),
        channel_mask: Some(3),
        sub_format: Some([
            0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x80, 0x00, 0x00, 0xaa, 0x00, 0x38,
            0x9b, 0x71,
        ]),
    });
    assert_eq!(chunk, expected_chunk);
}

#[test]
fn bad_fmt_ext_chunk() {
    let mut chunk_data = Bytes::from_static(&[
        0x66, 0x6d, 0x74, 0x20, 0x16, 0x00, 0x00, 0x00, 0x06, 0x00, 0x02, 0x00, 0x44, 0xac, 0x00,
        0x00, 0x88, 0x58, 0x01, 0x00, 0x02, 0x00, 0x08, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00,
    ]);
    assert_eq!(
        Chunk::pop_from_data(&mut chunk_data)
            .unwrap()
            .load_type()
            .unwrap_err(),
        ChunkError::FieldParse(FieldParseError {
            chunk_code: "fmt ".to_string(),
            field_name: "cbSize".to_string(),
            position: 18,
            reason: "Extension size mismatch. Reported: 0. Found: 4".to_string(),
        })
    );
}

#[test]
fn invalid_fmt_ext_chunk() {
    let mut chunk_data = Bytes::from_static(&[
        0x66, 0x6d, 0x74, 0x20, 0x16, 0x00, 0x00, 0x00, 0x06, 0x00, 0x02, 0x00, 0x44, 0xac, 0x00,
        0x00, 0x88, 0x58, 0x01, 0x00, 0x02, 0x00, 0x08, 0x00, 0x04, 0x00, 0x08, 0x00, 0x00, 0x00,
    ]);
    assert_eq!(
        Chunk::pop_from_data(&mut chunk_data)
            .unwrap()
            .load_type()
            .unwrap_err(),
        ChunkError::FieldParse(FieldParseError {
            chunk_code: "fmt ".to_string(),
            field_name: "cbSize".to_string(),
            position: 18,
            reason: "Invalid fmt extension size: 4".to_string(),
        })
    );
}

#[test]
fn info_list() {
    let mut chunk_data = Bytes::from_static(&[
        0x4c, 0x49, 0x53, 0x54, 0x1a, 0x0, 0x0, 0x0, 0x49, 0x4e, 0x46, 0x4f, 0x49, 0x53, 0x46,
        0x54, 0xd, 0x0, 0x0, 0x0, 0x4c, 0x61, 0x76, 0x66, 0x36, 0x31, 0x2e, 0x31, 0x2e, 0x31, 0x30,
        0x30, 0x0, 0x0,
    ]);
    let chunk = Chunk::pop_from_data(&mut chunk_data)
        .unwrap()
        .load_type()
        .unwrap();
    match chunk {
        ChunkType::List(list_chunk) => {
            let chunk_hashmap: Result<HashMap<String, String>, _> = list_chunk.try_into();
            assert!(chunk_hashmap.is_ok());
            assert_eq!(
                chunk_hashmap.unwrap(),
                HashMap::from([("Software".to_string(), "Lavf61.1.100".to_string())])
            );
        }
        _ => panic!("Not an Info chunk: {:?}", chunk),
    }
}
