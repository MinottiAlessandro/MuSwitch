use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

pub struct ApiEndpoints;

impl ApiEndpoints {
    pub const TOKEN: &'static str = "https://accounts.spotify.com/api/token";
    pub const GET_PLAYLIST_TRACKS: &'static str = "https://api.spotify.com/v1/playlists/{}/tracks";
    pub const GET_PLAYLISTS: &'static str = "https://api.spotify.com/v1/users/{}/playlists";
    pub const SEARCH: &'static str = "https://api.spotify.com/v1/search";
}

// --- Playlist Tracks ---
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
    images: Option<Vec<Image>>,
    pub name: String,
    release_date: Option<String>,
    release_date_precision: Option<String>,
    uri: String,
    pub artists: Vec<Artist>,
    external_urls: HashMap<String, String>,
    total_tracks: Option<usize>,
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
    height: Option<usize>,
    url: String,
    width: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VideoThumbnail {
    url: Option<String>,
}

// --- Playlist ---
#[derive(Debug, Deserialize, Serialize)]
pub struct ExternalUrls {
    spotify: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Owner {
    display_name: String,
    external_urls: ExternalUrls,
    href: String,
    id: String,
    #[serde(rename = "type")]
    type_: String,
    uri: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tracks {
    href: String,
    total: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlaylistItem {
    collaborative: bool,
    description: String,
    external_urls: ExternalUrls,
    href: String,
    pub id: String,
    images: Vec<Image>,
    pub name: String,
    owner: Owner,
    primary_color: Option<String>,
    public: bool,
    snapshot_id: String,
    tracks: Tracks,
    #[serde(rename = "type")]
    type_: String,
    uri: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Playlists {
    href: String,
    limit: u32,
    next: Option<String>,
    offset: u32,
    previous: Option<String>,
    total: u32,
    pub items: Vec<PlaylistItem>,
}

// --- Favourite ---
#[derive(Debug, Deserialize, Serialize)]
pub struct Restrictions {
    reason: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExternalIds {
    isrc: String,
    ean: String,
    upc: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Followers {
    href: Option<String>,
    total: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TrackArtist {
    external_urls: ExternalUrls,
    followers: Option<Followers>,
    genres: Option<Vec<String>>,
    href: String,
    id: String,
    images: Option<Vec<Image>>,
    name: String,
    popularity: Option<u32>,
    #[serde(rename = "type")]
    type_: String,
    uri: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Favourite {
    href: String,
    limit: u32,
    next: Option<String>,
    offset: u32,
    previous: Option<String>,
    total: u32,
    pub items: Vec<Item>,
}

// --- Search ---
#[derive(Debug, Serialize, Deserialize)]
pub struct Search {
    pub tracks: Albums,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Albums {
    href: String,
    pub items: Vec<Album>,
    limit: i32,
    next: Option<String>,
    offset: i32,
    previous: Option<String>,
    total: i32,
}
