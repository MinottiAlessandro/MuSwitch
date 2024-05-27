use crate::{models::spotify::Playlist, utils::web_interface::{AuthResponse, WebInterface}};
use std::env;
use dotenv::dotenv;
use serde::Deserialize;
use chrono::{DateTime, Utc, Duration};

#[derive(Deserialize, Debug)]
pub struct SpotifyAuthResponse {
    access_token: Option<String>,
    expires_in: Option<i64>,
}

impl AuthResponse for SpotifyAuthResponse {
    fn new() -> Self {
        SpotifyAuthResponse {
            access_token: Some(String::from("")),
            expires_in: Some(0),
        }
    }

    fn get_token(&mut self, retrieve_date: DateTime<Utc>) -> Option<String> {
        if let (Some(token), Some(expire_in)) = (&self.access_token, &self.expires_in) {
            if (retrieve_date + Duration::seconds(expire_in.clone())) > Utc::now() {
                return Some(token.clone());
            }
        }
        return None;
    }
}

pub struct SpotifyWebInterface {
    auth: SpotifyAuthResponse,
    auth_token_retrieve_date: Option<DateTime<Utc>>
}

impl WebInterface for SpotifyWebInterface {
    fn new() -> Self {
        return Self {
            auth: SpotifyAuthResponse::new(),
            auth_token_retrieve_date: Some(Utc::now())
        };
    }

    async fn fetch_token(&mut self, client: &reqwest::Client) -> Result<String, Box<dyn std::error::Error>> {
        if let Some(token) = self.auth.get_token(self.auth_token_retrieve_date.unwrap()) {
            return Ok(token);
        }

        dotenv().ok();

        let client_id = env::var("SPOTIFY_CLIENT_ID").expect("API_KEY must be set, create a .env file in the root of the project");
        let client_secret = env::var("SPOTIFY_CLIENT_SECRET").expect("SECRET_KEY must be set, create a .env file in the root of the project");

        let token_url = "https://accounts.spotify.com/api/token";
        let params = &[
            ("grant_type", String::from("client_credentials")),
            ("client_id", String::from(client_id)),
            ("client_secret", String::from(client_secret))
        ];

        self.auth = client
            .post(token_url)
            .form(&params)
            .send()
            .await?
            .json()
            .await?;
        self.auth_token_retrieve_date = Some(Utc::now());

        return Ok(self.auth.get_token(self.auth_token_retrieve_date.unwrap()).unwrap());
    }

    async fn get_playlist(&mut self, client: &reqwest::Client, playlist_id: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let access_token = self.fetch_token(&client).await?;

        let request_url = format!("https://api.spotify.com/v1/playlists/{}/tracks", playlist_id);
        let response_body: Playlist = client.get(request_url)
            .bearer_auth(&access_token)
            .send()
            .await?
            .json()
            .await?;

        let mut results: Vec<String> = Vec::new();
        for elem in response_body.items {
            let artists: Vec<String> = elem.track.artists.iter()
            .map(|artist| artist.name.to_string())
            .collect();

            results.push(artists.join("|") + ":" + &elem.track.name);
        }

        return Ok(results);
    }
}
