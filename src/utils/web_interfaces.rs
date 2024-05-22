use std::sync::Mutex;
use dotenv::dotenv;
use std::env;
use crate::models;

pub async fn fetch_token(client: &reqwest::Client, cache: &Mutex<models::spotify::AuthResponseCache>) -> Result<String, Box<dyn std::error::Error>> {
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

    let response: models::spotify::AuthResponse = client
        .post(token_url)
        .form(&params)
        .send()
        .await?
        .json()
        .await?;

    cache.set_token(response.get_token().to_string(), response.get_expiration());

    return Ok(response.get_token().to_string());
}
