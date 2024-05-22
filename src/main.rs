use std::sync::Mutex;
use std::io;
mod models;
mod utils;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // --- SPOTIFY ---
    let client = reqwest::Client::new();
    let token_cache = Mutex::new(models::spotify::AuthResponseCache::new());
    
    let access_token = utils::web_interfaces::fetch_token(&client, &token_cache).await?;

    loop {
        let mut playlist_id = String::new();

        println!("playlist ID (insert q to exit): ");
        
        io::stdin()
            .read_line(&mut playlist_id)
            .expect("Error reading input");
        
        playlist_id = playlist_id.trim().to_string();

        if playlist_id.to_lowercase() == "q".to_string() {
            println!("Exiting...");
            break;
        }

        println!("Getting the playlist: {playlist_id}");
        
        let request_url = format!("https://api.spotify.com/v1/playlists/{}/tracks", playlist_id);
        let response_body = client.get(request_url)
            .bearer_auth(&access_token)
            .send()
            .await?
            .text()
            .await?;

        let parsed_json: serde_json::Value = serde_json::from_str(&response_body)?;
        for i in 0..(parsed_json["total"].as_i64().unwrap() as usize) {
            println!("{} - {}", parsed_json["items"][i]["track"]["artists"][0]["name"], parsed_json["items"][i]["track"]["name"]);
        }
    }

    return Ok(());
}
