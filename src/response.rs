use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Album {
    pub url: String,
    pub name: String,
    #[serde(rename = "uploaderName")]
    pub uploader_name: String,
}

#[allow(dead_code)]
impl Album {
    fn new(url: String, name: String, uploader_name: String) -> Self {
        Self {
            url,
            name,
            uploader_name,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Artist {
    pub url: String,
    pub name: String,
}

#[allow(dead_code)]
impl Artist {
    fn new(url: String, name: String) -> Self {
        Self { url, name }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Song {
    pub url: String,
    #[serde(rename = "type")]
    pub video_type: String,
    pub title: Option<String>,
    pub duration: Option<i32>,
    #[serde(rename = "uploaderName")]
    pub uploader_name: Option<String>,
    pub video_duration: Option<String>,
    #[serde(rename = "isShort")]
    pub is_short: Option<bool>,
    #[serde(rename = "uploaderVerified")]
    pub uploader_verified: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SongResponse {
    pub items: Vec<Song>,
    pub nextpage: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArtistResponse {
    pub items: Vec<Artist>,
    pub nextpage: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AlbumResponse {
    pub items: Vec<Album>,
    pub nextpage: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrangeResult {
    pub title: String,
    pub url: String,
    pub duration: String,
    pub uploader: String,
    pub is_verified: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AlbumSongResponse {
    pub name: String,
    #[serde(rename = "relatedStreams")]
    pub related_streams: Vec<Song>,
}
