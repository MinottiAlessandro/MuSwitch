use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration};
use std::sync::Mutex;
use dotenv::dotenv;
use std::env;


#[derive(Deserialize, Debug)]
struct AuthResponse {
    access_token: String,
    token_type: String,
    expires_in: i64,
}


struct AuthResponseCache {
    access_token: Option<String>,
    token_type: Option<String>,
    expiration: Option<DateTime<Utc>>,
}


impl AuthResponseCache {
    fn new() -> Self {
        AuthResponseCache {
            access_token: None,
            token_type: None,
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


async fn fetch_token(client: &reqwest::Client, cache: &Mutex<AuthResponseCache>) -> Result<String, Box<dyn std::error::Error>> {
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

    cache.set_token(response.access_token.clone(), response.expires_in);

    return Ok(response.access_token);
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // --- SPOTIFY AUTH ---
    let client = reqwest::Client::new();
    let token_cache = Mutex::new(AuthResponseCache::new());
    
    let access_token = fetch_token(&client, &token_cache).await?;
    println!("{access_token}");

    return Ok(());
}
