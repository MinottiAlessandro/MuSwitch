use std::{env, collections::HashMap};
use crate::utils::interfaces::WebInterface;
use crate::models::youtube;
use dotenv::dotenv;

pub struct YouTubeWebInterface;

impl WebInterface for YouTubeWebInterface {
    fn new() -> Self {
        return Self;
    }

    async fn get_playlist_tracks(&mut self, playlist_id: &str) -> Result<std::collections::HashMap<String, Vec<String>>, Box<dyn std::error::Error>> {
        dotenv().ok();
        let api_key = env::var("YOUTUBE_API_KEY")
            .expect("API_KEY must be set, create a .env file in the root of the project")
            .to_string();
        let mut results: HashMap<String, Vec<String>> = HashMap::new();
        let request_url = youtube::ApiEndpoints::GET_PLAYLISTS_TRACKS;
        let client = reqwest::Client::new();
        let params = &[
            ("key", api_key),
            ("playlistId", String::from(playlist_id)),
            ("part", String::from("snippet")),
        ];

        let response_body: youtube::PlaylistItemListResponse = client.get(request_url)
            .query(&params)
            .send()
            .await?
            .json()
            .await?;

        for elem in response_body.items {
            results.insert(elem.snippet.title, vec![elem.snippet.videoOwnerChannelTitle]);
        }

        return Ok(results);
    }

    async fn get_playlists(&mut self, user_id: &str) -> Result<std::collections::HashMap<String, String>, Box<dyn std::error::Error>> {
        dotenv().ok();
        let mut results: HashMap<String, String> = HashMap::new();
        let api_key = env::var("YOUTUBE_API_KEY")
            .expect("API_KEY must be set, create a .env file in the root of the project")
            .to_string();
        let request_url = youtube::ApiEndpoints::GET_PLAYLISTS;
        let client = reqwest::Client::new();
        let params = &[
            ("key", api_key),
            ("channelId", String::from(user_id)),
            ("part", String::from("snippet")),
        ];

        let response_body: youtube::PlaylistListResponse = client.get(request_url)
            .query(&params)
            .send()
            .await?
            .json()
            .await?;

         for elem in response_body.items {
             results.insert(elem.id, elem.snippet.title);
         }

        return Ok(results);
    }

    async fn find_track(&mut self, song: &str, artists: Vec<&str>) -> Result<bool, Box<dyn std::error::Error>> {
        return Ok(true);
    }
}
