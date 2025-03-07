use rspotify::clients::OAuthClient;
use crate::AppState;
use serde::Serialize;
use axum::{ extract::State, Json };
use std::sync::Arc;

use crate::api::get_lyrics::get_lyrics;

#[derive(Serialize)]
pub struct SongResponse {
    pub song: String,
    pub artist: String,
    pub album: String,
    pub album_picture: String,
    pub progress: u32,
    pub lyrics: Vec<(String, u64)>,
}

pub async fn current_song(State(state): State<Arc<AppState>>) -> Result<
    Json<SongResponse>,
    Json<SongResponse>
> {
    let spotify = &state.spotify;
    let last_song = state.last_song.clone();

    let current_last_song = {
        let guard = last_song.read().await;
        guard.clone()
    };

    let current = spotify.current_playing(None, None::<Vec<_>>).await.map_err(|_| {
        Json(SongResponse {
            song: "".to_string(),
            artist: "".to_string(),
            album: "".to_string(),
            album_picture: "".to_string(),
            progress: 0,
            lyrics: Vec::new(),
        })
    })?;

    if let Some(current_item) = current {
        if let Some(rspotify::model::PlayableItem::Track(track)) = current_item.item {
            let id = track.id.clone();

            let lyrics = if current_last_song == track.name {
                Vec::new()
            } else if let Some(ref track_id) = id {
                let track_id_string = track_id.to_string();
                let track_id_str = track_id_string.split(':').last().unwrap_or("");

                get_lyrics(track_id_str).await.unwrap()
            } else {
                Vec::new()
            };

            {
                let mut write_guard = last_song.write().await;
                *write_guard = track.name.clone();
            }

            return Ok(
                Json(SongResponse {
                    song: track.name,
                    artist: track.artists
                        .get(0)
                        .map(|a| a.name.clone())
                        .unwrap_or_default(),
                    album: track.album.name,
                    album_picture: track.album.images
                        .get(0)
                        .map(|i| i.url.clone())
                        .unwrap_or_default(),
                    progress: current_item.progress.unwrap().num_milliseconds() as u32, // convert to milliseconds as u32
                    lyrics: lyrics,
                })
            );
        }
    }

    return Err(
        Json(SongResponse {
            song: "".to_string(),
            artist: "".to_string(),
            album: "".to_string(),
            album_picture: "".to_string(),
            progress: 0,
            lyrics: Vec::new(),
        })
    );
}
