use chrono::{DateTime, Utc};
use std::collections::HashMap;

pub trait WebInterface {
    fn new() -> Self;
    async fn fetch_token(&mut self, client: &reqwest::Client) -> Result<String, Box<dyn std::error::Error>>;
    async fn get_playlist(&mut self, client: &reqwest::Client, playlist_id: &str) -> Result<HashMap<String, Vec<String>>, Box<dyn std::error::Error>>;
}

pub trait AuthResponse: Sized + 'static {
    fn new() -> Self;
    fn get_token(&mut self, retrieve_date: DateTime<Utc>) -> Option<String>;
}
