use rspotify::ClientError;
use rspotify::prelude::*;
use std::error::Error;
use crate::AuthCodeSpotify;



pub async fn get_authorize_url(spotify: &AuthCodeSpotify) -> Result<String, Box<dyn Error>> {
    let url = spotify.get_authorize_url(true)?;
    Ok(url)
}

pub async fn handle_callback(spotify: &AuthCodeSpotify, code: &str) -> Result<(), ClientError> {
    spotify.request_token(code).await
}
