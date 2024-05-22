use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Playlist {
    href: String,
    pub items: Vec<Item>,
    limit: usize,
    next: Option<Value>,
    offset: usize,
    previous: Option<Value>,
    pub total: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    added_at: String,
    added_by: AddedBy,
    is_local: bool,
    primary_color: Option<Value>,
    pub track: Track,
    video_thumbnail: Option<VideoThumbnail>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AddedBy {
    external_urls: HashMap<String, String>,
    href: String,
    id: String,
    #[serde(rename = "type")]
    user_type: String,
    uri: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Track {
    preview_url: Option<String>,
    available_markets: Option<Vec<String>>,
    explicit: bool,
    #[serde(rename = "type")]
    track_type: String,
    episode: bool,
    track: bool,
    album: Album,
    pub artists: Vec<Artist>,
    disc_number: usize,
    track_number: usize,
    duration_ms: usize,
    external_ids: HashMap<String, String>,
    external_urls: HashMap<String, String>,
    href: String,
    id: String,
    pub name: String,
    popularity: usize,
    uri: String,
    is_local: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Album {
    available_markets: Option<Vec<String>>,
    #[serde(rename = "type")]
    album_type: String,
    href: String,
    id: String,
    images: Vec<Image>,
    name: String,
    release_date: String,
    release_date_precision: String,
    uri: String,
    artists: Vec<Artist>,
    external_urls: HashMap<String, String>,
    total_tracks: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Artist {
    external_urls: HashMap<String, String>,
    href: String,
    id: String,
    pub name: String,
    #[serde(rename = "type")]
    artist_type: String,
    uri: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Image {
    height: usize,
    url: String,
    width: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VideoThumbnail {
    url: Option<String>,
}