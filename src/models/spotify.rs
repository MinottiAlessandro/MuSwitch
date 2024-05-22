use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};

#[derive(Deserialize, Debug)]
pub struct AuthResponse {
    access_token: String,
    // token_type: String, // not used
    expires_in: i64,
}

impl AuthResponse {
    pub fn get_token(&self) -> &str {
        return &self.access_token;
    }

    pub fn get_expiration(&self) -> i64 {
        return self.expires_in;
    }
}

pub struct AuthResponseCache {
    access_token: Option<String>,
    // token_type: Option<String>, // not used
    expiration: Option<DateTime<Utc>>,
}

impl AuthResponseCache {
    pub fn new() -> Self {
        AuthResponseCache {
            access_token: None,
            // token_type: None, // not used
            expiration: None,
        }
    }

    pub fn get_token(&mut self) -> Option<String> {
        if let (Some(token), Some(expiration)) = (&self.access_token, &self.expiration) {
            if *expiration > Utc::now() {
                return Some(token.clone());
            }
        }
        return None;
    }

    pub fn set_token(&mut self, token: String, expires_in: i64) {
        self.access_token = Some(token);
        self.expiration = Some(Utc::now() + Duration::seconds(expires_in));
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Playlist {
    href: String,
    items: Vec<Item>,
    limit: usize,
    next: Option<Value>,
    offset: usize,
    previous: Option<Value>,
    total: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    added_at: String,
    added_by: AddedBy,
    is_local: bool,
    primary_color: Option<Value>,
    track: Track,
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
    artists: Vec<Artist>,
    disc_number: usize,
    track_number: usize,
    duration_ms: usize,
    external_ids: HashMap<String, String>,
    external_urls: HashMap<String, String>,
    href: String,
    id: String,
    name: String,
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
    name: String,
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