use reqwest::Client;
use std::error::Error;

pub async fn get_lyrics(track_id: &str) -> Result<Vec<(String, u64)>, Box<dyn Error>> {
    let url = format!("http://localhost:8001/?trackid={}", track_id);
    let response = Client::new().get(&url).send().await?;
    let body = response.text().await?;
    let json = serde_json::from_str::<serde_json::Value>(&body)?;

    let mut usable_data = Vec::new();
    for line in json["lines"].as_array().expect(&(json.to_string() + track_id)) {
        let words = line["words"].as_str().unwrap();
        let start_time: u64 = line["startTimeMs"].as_str().unwrap().parse().unwrap();
        if words.is_empty() {
            continue;
        }
        usable_data.push((words.to_string(), start_time));
    }
    Ok(usable_data)
}
