use crate::{models::spotify::Playlist, utils::web_interface::{AuthResponseCache, WebInterface}};
use std::sync::Mutex;
use std::env;
use dotenv::dotenv;
use serde::Deserialize;
use chrono::{DateTime, Utc, Duration};

#[derive(Deserialize, Debug)]
pub struct AuthResponse {
    access_token: String,
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

pub struct SpotifyAuthResponseCache {
    access_token: Option<String>,
    expiration: Option<DateTime<Utc>>,
}

impl AuthResponseCache for SpotifyAuthResponseCache {
    fn new() -> Self {
        SpotifyAuthResponseCache {
            access_token: None,
            expiration: None,
        }
    }

    fn get_token(&mut self) -> Option<String> {
        if let (Some(token), Some(expiration)) = (&self.access_token, &self.expiration) {
            if *expiration > Utc::now() {
                return Some(token.clone());
            }
        }
        return None;
    }

    fn set_token(&mut self, token: String, expires_in: i64) {
        self.access_token = Some(token);
        self.expiration = Some(Utc::now() + Duration::seconds(expires_in));
    }
}

pub struct SpotifyWebInterface;

impl WebInterface for SpotifyWebInterface {
    type Cache = SpotifyAuthResponseCache;

    async fn fetch_token(client: &reqwest::Client, cache: &Mutex<Self::Cache>) -> Result<String, Box<dyn std::error::Error>> {
        let mut cache = cache.lock().unwrap();

        if let Some(token) = cache.get_token() {
            return Ok(token);
        }

        dotenv().ok();

        let client_id = env::var("SPOTIFY_CLIENT_ID").expect("API_KEY must be set, create a .env file in the root of the project");
        let client_secret = env::var("SPOTIFY_CLIENT_SECRET").expect("SECRET_KEY must be set, create a .env file in the root of the project");

        // Replace this URL and parameters with your API's token endpoint
        let token_url = "https://accounts.spotify.com/api/token";
        let params = &[
            ("grant_type", String::from("client_credentials")), 
            ("client_id", String::from(client_id)), 
            ("client_secret", String::from(client_secret))
        ];

        let response: AuthResponse = client
            .post(token_url)
            .form(&params)
            .send()
            .await?
            .json()
            .await?;

        cache.set_token(response.get_token().to_string(), response.get_expiration());

        return Ok(response.get_token().to_string());
    }

    async fn get_playlist(client: &reqwest::Client, playlist_id: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let token_cache = Mutex::new(<SpotifyAuthResponseCache as AuthResponseCache>::new());
        let access_token = <SpotifyWebInterface as WebInterface>::fetch_token(&client, &token_cache).await?;
        
        let request_url = format!("https://api.spotify.com/v1/playlists/{}/tracks", playlist_id);
        let response_body: Playlist = client.get(request_url)
            .bearer_auth(&access_token)
            .send()
            .await?
            .json()
            .await?;
        
        let mut results: Vec<String> = Vec::new();
        for elem in response_body.items {    
            results.push(elem.track.name);
        }

        return Ok(results);
    }
}