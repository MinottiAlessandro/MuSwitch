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
    #[serde(skip_deserializing)]
    retrieve_date: Option<DateTime<Utc>>
}

impl AuthResponse for SpotifyAuthResponse {
    fn new() -> Self {
        SpotifyAuthResponse {
            access_token: Some(String::from("")),
            expires_in: Some(0),
            retrieve_date: Some(Utc::now())
        }
    }

    async fn fetch_token() -> Result<SpotifyAuthResponse, Box<dyn std::error::Error>> {
        dotenv().ok();

        let client_id = env::var("SPOTIFY_CLIENT_ID")
            .expect("API_KEY must be set, create a .env file in the root of the project")
            .to_string();
        let client_secret = env::var("SPOTIFY_CLIENT_SECRET")
            .expect("SECRET_KEY must be set, create a .env file in the root of the project")
            .to_string();

        let client = reqwest::Client::new();
        let token_url = spotify::ApiEndpoints::AUTH;
        let params = &[
            ("grant_type", String::from("client_credentials")),
            ("client_id", client_id),
            ("client_secret", client_secret)
        ];

        let mut result: SpotifyAuthResponse = client
            .post(token_url)
            .form(&params)
            .send()
            .await?
            .json()
            .await?;
        result.retrieve_date = Some(Utc::now());
        result.expires_in = Some(5);

        return Ok(result);
    }

    async fn get_token(&mut self) -> Option<String> {
        if (self.retrieve_date.unwrap() + Duration::seconds(self.expires_in.unwrap().clone())) > Utc::now() {
            return Some(self.access_token.clone().unwrap());
        }
        *self = SpotifyAuthResponse::fetch_token().await.unwrap();
        return Some(self.access_token.clone().unwrap());
    }
}

pub struct SpotifyWebInterface {
    auth: SpotifyAuthResponse,
}

impl WebInterface for SpotifyWebInterface {
    fn new() -> Self {
        return Self {
            auth: SpotifyAuthResponse::new()
        };
    }

    async fn get_playlist(&mut self, playlist_id: &str) -> Result<HashMap<String, Vec<String>>, Box<dyn std::error::Error>> {
        let access_token = self.auth.get_token().await.unwrap();

        let mut results: HashMap<String, Vec<String>> = HashMap::new();
        let request_url = spotify::ApiEndpoints::GET_PLAYLIST.replace("{}", playlist_id);
        let client = reqwest::Client::new();
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
