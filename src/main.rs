use clap::{App, Arg};
use utils::{spotify_interface::SpotifyWebInterface, interfaces::WebInterface};
mod models;
mod utils;

fn init_cli() -> App<'static> {
    return App::new("MuSwitch")
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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = init_cli();

    if app.clone().get_matches().is_present("help") {
        app.print_help().unwrap();
        return Ok(());
    }

    let mut sc: SpotifyWebInterface = SpotifyWebInterface::new();
    let playlist_id = app.get_matches().value_of("playlistID").unwrap().to_string();
    let results = sc.get_playlist(&playlist_id).await;

    for (song, artists) in results.unwrap() {
        println!("song: {}, artists: {:?}", song, artists);
    }

    return Ok(());
}
