use clap::{App, Arg};
use utils::{interfaces::WebInterface, youtube_interface::YouTubeWebInterface, spotify_interface::SpotifyWebInterface};
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

    let s = "Ramaya";
    let a = vec!["Afrik Simon"];
    // let user_id = app.get_matches().value_of("userID").unwrap().to_string();
    let mut sc = SpotifyWebInterface::new();
    sc.find_track(s, a).await?;
    // let playlists = sc.get_playlists(&user_id).await?;

    // for (id, name) in playlists {
    //     let songs = sc.get_playlist_tracks(&id).await?;
    //     for (song, artist) in songs {
    //         println!("{}", song);
    //     }
    // }

    // let mut yc = YouTubeWebInterface::new();
    // let playlists = yc.get_playlists(&user_id).await?;

    // for (id, playlist) in playlists {
    //     println!("{} - {}", id, playlist);
    //     let songs = yc.get_playlist_tracks(&id).await?;
    //     for (song, artists) in songs {
    //         println!("\t{} - {:?}", song, artists);
    //     }
    // }

    return Ok(());
}
