use std::collections::HashMap;

pub trait WebInterface {
    fn new() -> Self;
    async fn get_playlist_tracks(&mut self, playlist_id: &str) -> Result<HashMap<String, Vec<String>>, Box<dyn std::error::Error>>;
    async fn get_playlists(&mut self, user_id: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>>;
}

pub trait AuthResponse: Sized + 'static {
    fn new() -> Self;
    async fn fetch_token() -> Result<Self, Box<dyn std::error::Error>>;
    async fn get_token(&mut self) -> Option<String>;
}
