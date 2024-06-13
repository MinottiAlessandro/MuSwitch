use serde::{Deserialize, Serialize};

pub struct ApiEndpoints;

impl ApiEndpoints {
    pub const GET_PLAYLISTS: &'static str = "https://www.googleapis.com/youtube/v3/playlists";
    pub const GET_PLAYLISTS_TRACKS: &'static str = "https://www.googleapis.com/youtube/v3/playlistItems";
}

// --- Playlist Tracks ---
#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct PlaylistItemListResponse {
    kind: String,
    etag: String,
    pub items: Vec<PlaylistItem>,
    pageInfo: PageInfo,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PlaylistItem {
    kind: String,
    etag: String,
    pub id: String,
    pub snippet: Snippet,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct Snippet {
    publishedAt: String,
    channelId: String,
    pub title: String,
    description: String,
    thumbnails: Thumbnails,
    channelTitle: String,
    pub playlistId: String,
    position: u32,
    resourceId: ResourceId,
    pub videoOwnerChannelTitle: String,
    pub videoOwnerChannelId: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Thumbnails {
    default: Thumbnail,
    medium: Thumbnail,
    high: Thumbnail,
    standard: Thumbnail,
    maxres: Thumbnail,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Thumbnail {
    url: String,
    width: u32,
    height: u32,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct ResourceId {
    kind: String,
    pub videoId: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct PageInfo {
    totalResults: u32,
    resultsPerPage: u32,
}

// --- Playlists ---
#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct PlaylistListResponse {
    kind: String,
    etag: String,
    pageInfo: PageInfo,
    pub items: Vec<Playlist>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Playlist {
    kind: String,
    etag: String,
    pub id: String,
    pub snippet: PlaylistSnippet,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct PlaylistSnippet {
    publishedAt: String,
    channelId: String,
    pub title: String,
    description: String,
    thumbnails: Thumbnails,
    pub channelTitle: String,
    localized: Localized,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Localized {
    title: String,
    description: String,
}
