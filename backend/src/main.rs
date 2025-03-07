mod api;
use api::{ current_song, login };

use rspotify::{ scopes, AuthCodeSpotify, Config };
use axum::{ routing::get, serve::serve, Router };
use tokio::{ net::TcpListener, sync::RwLock };
use std::sync::Arc;
use std::net::SocketAddr;
use tower_http::cors::{ Any, CorsLayer };

#[derive(Clone)]
pub struct AppState {
    spotify: AuthCodeSpotify,
    last_song: Arc<RwLock<String>>,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    std::process::Command
        ::new("sh")
        .arg("-c")
        .arg("php -S localhost:8001 spotify-lyrics-api/api/index.php > /dev/null 2>&1 &")
        .spawn()
        .expect("Failed to start PHP server");

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
        last_song: Arc::new(RwLock::new("".to_string())),
    });

    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);

    let app = Router::new()
        .route("/", get(login::get_authorized_url))
        .route("/current_song", get(current_song::current_song))
        .route("/callback", get(login::callback))
        .with_state(state)
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Listening on http://{}", addr);

    serve(listener, app).await?;

    Ok(())
}
