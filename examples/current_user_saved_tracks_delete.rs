extern crate rspotify;

use rspotify::spotify::client::Spotify;
use rspotify::spotify::util::get_token;
use rspotify::spotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};

fn main() {
    // Set client_id and client_secret in .env file or
    // export CLIENT_ID="your client_id"
    // export CLIENT_SECRET="secret"
    // export REDIRECT_URI=your-direct-uri

    // Or set client_id, client_secret,redirect_uri explictly
    // let oauth = SpotifyOAuth::default()
    //     .client_id("this-is-my-client-id")
    //     .client_secret("this-is-my-client-secret")
    //     .redirect_uri("http://localhost:8888/callback")
    //     .build();

    let mut oauth = SpotifyOAuth::default().scope("user-library-modify").build();
    match get_token(&mut oauth) {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            // Or set client_id and client_secret explictly
            // let client_credential = SpotifyClientCredentials::default()
            //     .client_id("this-is-my-client-id")
            //     .client_secret("this-is-my-client-secret")
            //     .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            //this is my(samray's) user_id and playlist_id, so just change
            // user_id and playlist_id to yours, or you will get a 403 forbidden error
            let mut tracks_ids = vec![];
            let track_id1 = String::from("spotify:track:4iV5W9uYEdYUVa79Axb7Rh");
            let track_id2 = String::from("spotify:track:1301WleyT98MSxVHPZCA6M");
            tracks_ids.push(track_id2);
            tracks_ids.push(track_id1);
            match spotify.current_user_saved_tracks_delete(&tracks_ids) {
                Ok(_) => {
                    println!("saved tracks delete successful");
                }
                Err(_) => eprintln!("saved tracks delete failed"),

            }
        }
        None => println!("auth failed"),
    };

}
