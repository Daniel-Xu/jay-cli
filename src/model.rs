use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Serialize, Deserialize)]
pub struct JayMusic {
    pub list: Vec<Song>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Album {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SongInfo {
    #[serde(rename = "128")]
    pub addr_128: String,
    #[serde(rename = "320")]
    pub addr_320: String,
    #[serde(rename = "flac", default)]
    pub addr_flac: String,
    pub duration: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Song {
    pub id: String,
    pub name: String,
    pub album: Album,
    #[serde(rename = "songInfo")]
    pub info: SongInfo,
}

impl Display for Song {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
