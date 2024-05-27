use crate::{models::spotify, utils::interfaces::{AuthResponse, WebInterface}};
use std::env;
use std::collections::HashMap;
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
    auth_token_retrieve_date: DateTime<Utc>
}

impl WebInterface for SpotifyWebInterface {
    fn new() -> Self {
        return Self {
            auth: SpotifyAuthResponse::new(),
            auth_token_retrieve_date: Utc::now()
        };
    }

    async fn fetch_token(&mut self, client: &reqwest::Client) -> Result<String, Box<dyn std::error::Error>> {
        if let Some(token) = self.auth.get_token(self.auth_token_retrieve_date) {
            return Ok(token);
        }

        dotenv().ok();

        let client_id = env::var("SPOTIFY_CLIENT_ID").expect("API_KEY must be set, create a .env file in the root of the project");
        let client_secret = env::var("SPOTIFY_CLIENT_SECRET").expect("SECRET_KEY must be set, create a .env file in the root of the project");

        let token_url = spotify::ApiEndpoints::AUTH;
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
        self.auth_token_retrieve_date = Utc::now();

        return Ok(self.auth.get_token(self.auth_token_retrieve_date).unwrap());
    }

    async fn get_playlist(&mut self, client: &reqwest::Client, playlist_id: &str) -> Result<HashMap<String, Vec<String>>, Box<dyn std::error::Error>> {
        let access_token = self.fetch_token(&client).await?;

        let mut results: HashMap<String, Vec<String>> = HashMap::new();
        let request_url = spotify::ApiEndpoints::GET_PLAYLIST.replace("{}", playlist_id);
        let response_body: spotify::Playlist = client.get(request_url)
            .bearer_auth(&access_token)
            .send()
            .await?
            .json()
            .await?;

        for elem in response_body.items {
            results.insert(elem.track.name, elem.track.artists.iter()
                .map(|artist| artist.name.to_string())
                .collect()
            );
        }

        return Ok(results);
    }
}
