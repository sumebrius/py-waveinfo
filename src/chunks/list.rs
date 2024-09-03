use core::str;
use std::{collections::HashMap, str::FromStr};
use strum::EnumMessage;
use strum_macros::{EnumMessage, EnumString};

use super::Chunk;
use crate::errors::ChunkError;

#[derive(Debug, PartialEq)]
pub(crate) struct List {
    list_type: String,
    subchunks: Vec<Chunk>,
}

impl TryFrom<Chunk> for List {
    type Error = ChunkError;

    fn try_from(mut chunk: Chunk) -> Result<Self, Self::Error> {
        chunk.validate_type("LIST")?;

        let list_type = chunk.data_string::<4>("LIST")?;
        let subchunks = chunk.collect::<Result<Vec<Chunk>, ChunkError>>()?;

        Ok(Self {
            list_type,
            subchunks,
        })
    }
}

impl TryInto<HashMap<String, String>> for List {
    type Error = ();

    fn try_into(self) -> Result<HashMap<String, String>, Self::Error> {
        match self.list_type.as_str() {
            "INFO" => Ok(self
                .subchunks
                .into_iter()
                .flat_map(info_tuple)
                .collect::<HashMap<String, String>>()),
            // INFO seems to be the only defined list type?
            _ => Err(()),
        }
    }
}

#[derive(Debug, EnumMessage, EnumString, PartialEq)]
#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
pub(crate) enum InfoChunk {
    #[strum(message = "Archival Location")]
    IARL,
    #[strum(message = "Artist")]
    IART,
    #[strum(message = "Commissioned")]
    ICMS,
    #[strum(message = "Comments")]
    ICMT,
    #[strum(message = "Copyright")]
    ICOP,
    #[strum(message = "Creation date")]
    ICRD,
    #[strum(message = "Cropped")]
    ICRP,
    #[strum(message = "Dimensions")]
    IDIM,
    #[strum(message = "Dots Per Inch")]
    IDPI,
    #[strum(message = "Engineer")]
    IENG,
    #[strum(message = "Genre")]
    IGNR,
    #[strum(message = "Keywords")]
    IKEY,
    #[strum(message = "Lightness")]
    ILGT,
    #[strum(message = "Medium")]
    IMED,
    #[strum(message = "Name")]
    INAM,
    #[strum(message = "Palette Setting")]
    IPLT,
    #[strum(message = "Product")]
    IPRD,
    #[strum(message = "Subject")]
    ISBJ,
    #[strum(message = "Software")]
    ISFT,
    #[strum(message = "Sharpness")]
    ISHP,
    #[strum(message = "Source")]
    ISRC,
    #[strum(message = "Source Form")]
    ISRF,
    #[strum(message = "Technician")]
    ITCH,
}

fn info_tuple(mut chunk: Chunk) -> Option<(String, String)> {
    if let Ok(infochunk) = InfoChunk::from_str(&chunk.id) {
        if let Ok(value) = chunk.data_zstring("Info value") {
            return Some((
                infochunk
                    .get_message()
                    .expect("Info type without label")
                    .to_string(),
                value,
            ));
        }
    };
    None
}
