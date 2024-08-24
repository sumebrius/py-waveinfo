use std::collections::HashMap;

use bytes::Bytes;

use super::*;

#[test]
fn test_pop_from_data() {
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
fn test_pop_with_padding_byte() {
    let mut chunk_data = Bytes::from_static(&[
        0x54, 0x45, 0x53, 0x54, 0x03, 0x00, 0x00, 0x00, 0x44, 0x41, 0x54, 0x00, 0x42, 0x41, 0x54,
    ]);
    let chunk = Chunk::pop_from_data(&mut chunk_data).unwrap();
    assert_eq!(chunk.size, 3);
    assert_eq!(chunk.data, Bytes::from_static(&[0x44, 0x41, 0x54]));
    assert_eq!(chunk_data, Bytes::from_static(&[0x42, 0x41, 0x54]));
}

#[test]
fn test_data_bytes() {
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
fn test_pop_zstring() {
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
fn test_info_list() {
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
