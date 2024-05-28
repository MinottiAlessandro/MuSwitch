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
            .required(false)
            .takes_value(true)
            .help("Playlist ID"),
    )
    .arg(
        Arg::with_name("userID")
            .short('u')
            .long("user")
            .required(true)
            .takes_value(true)
            .help("User ID")
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
    //let playlist_id = app.get_matches().value_of("playlistID").unwrap().to_string();
    //let user_id = app.get_matches().value_of("userID").unwrap().to_string();
    //let results = sc.get_playlists(&user_id).await;

    //for (id, name) in results.unwrap() {
    //    let songs = sc.get_playlist_tracks(&id).await;
    //    println!("--- {} ---", name);
    //    for (song, artists) in songs.unwrap() {
    //        println!("{} - {:?}", song, artists);
    //    }
    //}

    let result = sc.get_fav_tracks().await;

    for (song, artists) in result.unwrap() {
        println!("{} - {:?}", song, artists);
    }

    return Ok(());
}
