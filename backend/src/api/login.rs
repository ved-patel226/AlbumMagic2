use axum::response::Redirect;
use axum::{ response::IntoResponse, Json };
use serde::{ Serialize, Deserialize };
use crate::AppState;
use axum::extract::{ State, Query };
use std::sync::Arc;
use rspotify::clients::OAuthClient;
use rspotify::clients::BaseClient;

#[derive(Serialize)]
struct AuthorizeURLOUTPUT {
    url: String,
}

#[derive(Serialize)]
struct CallbackOutput {
    success: bool,
}

#[derive(Deserialize)]
pub struct CallbackParams {
    pub code: String,
}

pub async fn get_authorized_url(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let spotify = &state.spotify;

    let url = spotify.get_authorize_url(true).unwrap();

    Json(AuthorizeURLOUTPUT { url })
}

pub async fn callback(
    State(state): State<Arc<AppState>>,
    Query(params): Query<CallbackParams>
) -> impl IntoResponse {
    let spotify = &state.spotify;

    spotify.write_token_cache().await.unwrap();

    if spotify.get_token().lock().await.unwrap().is_some() {
        return Redirect::temporary("http://localhost:5173/").into_response();
    }

    match spotify.request_token(&params.code).await {
        Ok(_) => Redirect::temporary("http://localhost:5173/").into_response(),
        Err(_) => Json(CallbackOutput { success: false }).into_response(),
    }
}
