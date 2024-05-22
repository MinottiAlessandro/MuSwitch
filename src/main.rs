use clap::{App, Arg};
use utils::{spotify_web_interface::SpotifyWebInterface, web_interface::WebInterface};
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
    
    let client = reqwest::Client::new();
    let playlist_id = app.get_matches().value_of("playlistID").unwrap().to_string();
    let results = SpotifyWebInterface::get_playlist(&client, playlist_id).await?;

    for song in results {
        println!("{}", song);
    }

    return Ok(());
}
