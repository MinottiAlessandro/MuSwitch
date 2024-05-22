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
        let response_body: models::spotify::Playlist = client.get(request_url)
            .bearer_auth(&access_token)
            .send()
            .await?
            .json()
            .await?;

        for elem in response_body.items {
            let mut artists: Vec<String> = Vec::new();
            for artist in elem.track.artists {
                artists.push(artist.name.to_string());
            }
            println!("{} -> {}", artists.join(", "), elem.track.name);
        }
    }

    return Ok(());
}
