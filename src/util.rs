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
