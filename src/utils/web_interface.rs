use std::sync::Mutex;

pub trait WebInterface {
    type Cache: AuthResponseCache;

    async fn fetch_token(client: &reqwest::Client, cache: &Mutex<Self::Cache>) -> Result<String, Box<dyn std::error::Error>>;
    async fn get_playlist(client: &reqwest::Client, playlist_id: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
}

pub trait AuthResponseCache: Sized + 'static {
    fn new() -> Self;
    fn get_token(&mut self) -> Option<String>;
    fn set_token(&mut self, token: String, expires_in: i64);
}
