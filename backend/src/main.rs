use std::env;

mod api;
use api::{current_song, login};

use clearscreen::clear;
use rascii_art::{render_to, RenderOptions};
use rspotify::AuthCodeSpotify;
use terminal_size::{terminal_size, Width};
use termion::{color, style};
use unicode_width::UnicodeWidthStr;

fn render_album_art(img_path: &str, width: usize) -> Result<String, Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    render_to(
        img_path,
        &mut buffer,
        &RenderOptions::new()
            .width(50)
            .colored(true)
            .charset(&[" ", "░", "▒", "▓", "█"]),
    )?;


    Ok(buffer)
}
fn create_progress_bar(progress: f32, width: usize) -> String {
    let filled_width = (progress * width as f32) as usize;
    let empty_width = width - filled_width;
    
    format!(
        "{}{}{}{}{}",
        color::Fg(color::LightGreen),
        "━".repeat(filled_width),
        color::Fg(color::LightBlack),
        "━".repeat(empty_width),
        color::Fg(color::Reset)
    )
}


fn center_text(text: &str, width: usize) -> String {
    let text_width = UnicodeWidthStr::width(text);
    if text_width >= width {
        return text.to_string();
    }
    
    let padding = (width - text_width) / 2;
    format!("{}{}{}", " ".repeat(padding), text, " ".repeat(width - text_width - padding))
}

async fn display_song_info(
    spotify: &AuthCodeSpotify,
    cached_album_url: &mut String,
    cached_art: &mut String,
) -> Result<bool, Box<dyn std::error::Error>> {
    let song = current_song(&spotify).await?;
    let mut updated = false;

    // Get terminal size
    let size = terminal_size();
    let width = size.map(|(Width(w), _)| w as usize).unwrap_or(80);
    

    if song.album_picture != *cached_album_url {
        let image = reqwest::get(&song.album_picture).await?.bytes().await?;
        let temp_path = "/tmp/temp_album.jpg";
        std::fs::write(temp_path, &image)?;
        *cached_art = render_album_art(temp_path, width)?;
        *cached_album_url = song.album_picture.clone();
        updated = true;
    }


    let progress = 0.65;
    let progress_bar = create_progress_bar(progress, width - 20);
    
    let current_time = "2:15";
    let total_time = "3:45";

    clear()?;
    
    println!("\n{}{}{} SPOTIFY NOW PLAYING {}{}{}\n", 
        color::Fg(color::Green), 
        style::Bold,
        "✨",
        "✨",
        style::Reset,
        color::Fg(color::Reset)
    );
    
    println!("{}", cached_art);
    
    println!("\n{}{}{}{}", 
        color::Fg(color::LightGreen), 
        style::Bold,
        center_text(&song.song, width),
        style::Reset
    );
    
    println!("{}{}{}", 
        color::Fg(color::LightYellow),
        center_text(&format!("by {}", song.artist), width),
        color::Fg(color::Reset)
    );
    
    println!("{}{}{}", 
        color::Fg(color::LightCyan),
        center_text(&format!("from {}", song.album), width),
        color::Fg(color::Reset)
    );
    
    println!("\n{} {} {}", 
        current_time,
        progress_bar,
        total_time
    );
    
    println!("\n{}{}", 
        color::Fg(color::LightBlack),
        center_text("Press 'q' to quit | 'n' next | 'p' previous | 'space' play/pause", width)
    );

    Ok(updated)
}

async fn main_loop(spotify: &AuthCodeSpotify) -> Result<(), Box<dyn std::error::Error>> {
    let mut cached_album_url = String::new();
    let mut cached_art = String::new();

    loop {
        display_song_info(spotify, &mut cached_album_url, &mut cached_art).await?;
        
        // Check for user input (non-blocking)
        if let Some(event) = check_for_input() {
            match event {
                'q' => break, // Quit
                'n' => {} // Next song (implement with Spotify API)
                'p' => {} // Previous song (implement with Spotify API)
                ' ' => {} // Play/pause (implement with Spotify API)
                _ => {}
            }
        }
        
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    Ok(())
}

// Mock function for checking input - you'll need to implement this
fn check_for_input() -> Option<char> {
    // In a real implementation, use termion or similar to check for keypresses
    None
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Print welcome message
    clear()?;
    println!("\n{}{} Welcome to Spotify CLI Visualizer {}{}\n", 
        color::Fg(color::Green),
        style::Bold,
        style::Reset,
        color::Fg(color::Reset)
    );

    // Initialize Spotify client
    let client_id = env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID must be set");
    let client_secret = env::var("SPOTIFY_CLIENT_SECRET").expect("SPOTIFY_CLIENT_SECRET must be set");
    let redirect_uri = env::var("SPOTIFY_REDIRECT_URI").expect("SPOTIFY_REDIRECT_URI must be set");

    let scopes = rspotify::scopes!("user-read-currently-playing", "user-read-playback-state");

    let creds = rspotify::Credentials::new(client_id.as_str(), client_secret.as_str());
    let oauth = rspotify::OAuth {
        scopes,
        redirect_uri,
        ..Default::default()
    };
    let spotify = AuthCodeSpotify::new(creds, oauth);

    // Request and print authorization URL with styling
    let authorize_url = login::get_authorize_url(&spotify)
        .await
        .expect("Failed to get authorization URL");
    
    println!("{}{}Please visit this URL to authorize the application:{}{}", 
        color::Fg(color::Yellow), 
        style::Bold,
        style::Reset,
        color::Fg(color::Reset)
    );
    println!("{}{}\n", color::Fg(color::LightBlue), authorize_url);

    // Prompt user for the redirect URL which contains the authorization code
    println!("{}After authorizing, please paste the full redirect URL:{}", 
        color::Fg(color::Yellow),
        color::Fg(color::Reset)
    );
    
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let input = input.trim();

    // Extract the authorization code from the URL query parameters
    let code = input
        .split("code=")
        .nth(1)
        .and_then(|s| s.split('&').next())
        .expect("Authorization code not found in the URL");

    println!("\n{}Connecting to Spotify...{}", 
        color::Fg(color::LightGreen),
        color::Fg(color::Reset)
    );
    
    login::handle_callback(&spotify, code).await?;
    
    println!("\n{}Connected! Loading your currently playing track...{}", 
        color::Fg(color::LightGreen),
        color::Fg(color::Reset)
    );
    
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    
    main_loop(&spotify).await?;

    Ok(())
}