mod api;
use api::{ current_song, login };

use rspotify::{ AuthCodeSpotify, scopes, Config };
use axum::{ routing::get, serve::serve, Router };
use tokio::net::TcpListener;
use std::sync::Arc;
use std::net::SocketAddr;

#[derive(Clone)]
pub struct AppState {
    spotify: AuthCodeSpotify,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    let scopes = scopes!("user-read-currently-playing");

    let client_id = std::env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID must be set");
    let client_secret = std::env
        ::var("SPOTIFY_CLIENT_SECRET")
        .expect("SPOTIFY_CLIENT_SECRET must be set");
    let redirect_uri = std::env
        ::var("SPOTIFY_REDIRECT_URI")
        .expect("SPOTIFY_REDIRECT_URI must be set");

    let creds = rspotify::Credentials::new(client_id.as_str(), client_secret.as_str());
    let oauth = rspotify::OAuth {
        scopes,
        redirect_uri,
        ..Default::default()
    };
    let config = Config {
        token_cached: true,
        cache_path: std::path::PathBuf::from(".spotify_token_cache.json"),
        token_refreshing: true,
        ..Default::default()
    };
    let spotify = AuthCodeSpotify::with_config(creds, oauth, config);

    let state = Arc::new(AppState {
        spotify: spotify,
    });

    let app = Router::new()
        .route("/", get(login::get_authorized_url))
        .route("/current_song", get(current_song::current_song))
        .route("/callback", get(login::callback))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Listening on http://{}", addr);

    serve(listener, app).await?;

    Ok(())
}
