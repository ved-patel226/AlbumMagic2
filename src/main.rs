use std::sync::Arc;
mod api;

use api::{current_song, login::{callback, login_handler}};

use axum::routing::{get, Router};

use rspotify::{
    scopes, AuthCodeSpotify, Credentials, OAuth,
};
use tokio::sync::Mutex;
use serde::Deserialize;

#[derive(Deserialize)]
struct CallbackQuery {
    code: String,
}


#[derive(Clone)]
struct AppState {
    spotify: Arc<Mutex<AuthCodeSpotify>>,
}


#[tokio::main]
async fn main() {
    // Initialize logger and load credentials from env variables.
    env_logger::init();
    let creds = Credentials::from_env().unwrap();

    // Define scopes needed for the API.
    let scopes = scopes!(
        // "user-read-email",
        // "user-read-private",
        // "user-top-read",
        // "user-read-recently-played",
        // "user-follow-read",
        // "user-library-read",
        "user-read-currently-playing"
        // "user-read-playback-state",
        // "user-read-playback-position",
        // "playlist-read-collaborative",
        // "playlist-read-private",
        // "user-follow-modify",
        // "user-library-modify",
        // "user-modify-playback-state",
        // "playlist-modify-public",
        // "playlist-modify-private",
        // "ugc-image-upload"
    );
    let oauth = OAuth::from_env(scopes).unwrap();
    let spotify = AuthCodeSpotify::new(creds, oauth);

    // Wrap the spotify instance in shared state.
    let state = AppState {
        spotify: Arc::new(Mutex::new(spotify)),
    };

    // Build the Axum router with two routes.
    let app = Router::new()
        .route("/current-song", get(current_song))
        .route("/callback", get(callback))
        .route("/", get(login_handler))
        .with_state(state);


        println!("Server running on http://127.0.0.1:5000/");

        let listener = tokio::net::TcpListener::bind("127.0.0.1:5000").await.unwrap();
        axum::serve(listener, app.into_make_service()).await.unwrap();
}

