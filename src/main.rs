use std::sync::Mutex;
use clap::{App, Arg};
mod models;
mod utils;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new("My Program")
    .version("0.1.0")
    .author("Alessandro Minotti")
    .about("A tool to copy playlist")
    .arg(
        Arg::with_name("playlistID")
            .short('p')
            .long("playlist")
            .required(true)
            .takes_value(true)
            .help("Playlist to retrieve"),
    );

    if app.clone().get_matches().is_present("help") {
        app.print_help().unwrap();
        return Ok(());
    }

    let playlist_id = app.get_matches().value_of("playlistID").unwrap().to_string();

    // --- SPOTIFY ---
    let client = reqwest::Client::new();
    let token_cache = Mutex::new(models::spotify::AuthResponseCache::new());
    let access_token = utils::web_interfaces::fetch_token(&client, &token_cache).await?;

    println!("Getting the playlist: {playlist_id}");
    
    let request_url = format!("https://api.spotify.com/v1/playlists/{}/tracks", playlist_id);
    let response_body: models::spotify::Playlist = client.get(request_url)
        .bearer_auth(&access_token)
        .send()
        .await?
        .json()
        .await?;

    for elem in response_body.items {
        let artists: Vec<String> = elem.track.artists.iter()
            .map(|artist| artist.name.to_string())
            .collect();

        println!("{} -> {}", artists.join(", "), elem.track.name);
    }

    return Ok(());
}
