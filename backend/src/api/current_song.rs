use rspotify::clients::OAuthClient;
use crate::AuthCodeSpotify;
use serde::Serialize;
use anyhow::{Context, Result};

#[derive(Serialize)]
pub struct SongResponse {
    pub song: String,
    pub artist: String,
    pub album: String,
    pub album_picture: String,
}


pub async fn current_song(spotify: &AuthCodeSpotify) -> Result<SongResponse> {
    let current = spotify
        .current_playing(None, None::<Vec<_>>)
        .await
        .context("Failed fetching current playing track")?;

    if let Some(current_item) = current {
        if let Some(rspotify::model::PlayableItem::Track(track)) = current_item.item {
            return Ok(SongResponse {
                song: track.name,
                artist: track.artists.get(0).map(|a| a.name.clone()).unwrap_or_default(),
                album: track.album.name,
                album_picture: track.album.images.get(0).map(|i| i.url.clone()).unwrap_or_default(),
            });
        } else {
            anyhow::bail!("The current item is not a track");
        }
    }

    anyhow::bail!("No current playing track found")
}
