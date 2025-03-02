use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT};
use std::error::Error;


async fn get_access_token() -> Result<String, Box<dyn Error>> {
    let url = "https://open.spotify.com/get_access_token?reason=transport&productType=web_player";
    let mut headers = HeaderMap::new();
    headers.insert("Cookie", HeaderValue::from_static(
        "sp_dc=AQDuAXCGynUrsI8WheBZycqsXhpk6AZMtppMahBfAiNBglkNLk6zSnYgWxTgla4kPXW_S4QW8rgGD_FL4nBBWp-z4s1K6L5G3CwwnY-mvItR2utaQnm_7gChVKIN9vja5tZmTo3h4HbTcMz8GFBtizmRtB-d_Que4F3egGe-wl7atS_rjmL4l9o0XWsZa3FXvp8Ya2mc6EO8zY0Tj9k; sp_key=f17255c2-9291-48cb-9a84-cdc50fa42980; sp_adid=1cc446ff-2329-4533-bac3-0b1f2756112e; sp_t=57be40f43c9d717e441272fc53bfea15; _ga=GA1.1.711506946.1739664945; _ga_BMC5VGR8YS=GS1.2.1740794265.2.0.1740794265.60.0.0; _ga_ZWG1NSHWD8=GS1.1.1740794265.2.0.1740794266.0.0.0; sp_m=us; sp_gaid=0088fcc8181e75245d4299a1fda0db72a9d0dc6db7ce2941ab3bee; _ga_S0T2DJJFZM=GS1.1.1740798836.1.0.1740798836.0.0.0; _scid=E5U2H5lT3dMCA8aPWS5UwS-q8pzlQ1-g; _scid_r=E5U2H5lT3dMCA8aPWS5UwS-q8pzlQ1-g; _cs_c=0; _cs_id=8fc08134-757b-a49d-bd5c-5af6b2852a8c.1740798836.1.1740798836.1740798836.1739197584.1774962836615.1.x; OptanonConsent=isGpcEnabled; _ga_ZWRF3NLZJZ=GS1.1.1740938310.4.1.1740939254.0.0.0"
    ));

    let response = Client::new()
        .get(url)
        .headers(headers)
        .send()
        .await?;
    
    let body = response.text().await?;
    let json = serde_json::from_str::<serde_json::Value>(&body)?;
    Ok(json["accessToken"].as_str().unwrap().to_string())
}

pub async fn get_lyrics(track_id: &str) -> Result<Vec<(String, u64)>, Box<dyn Error>> {
    let access_token = get_access_token().await?;

    let url = format!(
        "https://spclient.wg.spotify.com/color-lyrics/v2/track/{}?format=json&market=from_token",
        track_id
    );

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static(
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.0.0 Safari/537.36"));
    headers.insert("App-platform", HeaderValue::from_static("WebPlayer"));
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", access_token))?);

    let client = Client::new();
    let response = client
        .get(&url)
        .headers(headers)
        .send()
        .await?;
    
    let body = response.text().await?;
    let json = serde_json::from_str::<serde_json::Value>(&body)?;

    let mut usable_data: Vec<(String, u64)> = Vec::new();

    for line in json["lyrics"]["lines"].as_array().unwrap() {
        let words = line["words"].as_str().unwrap().to_string();
        let start_time = line["startTimeMs"].as_str().unwrap().parse().unwrap();

        if words == "" {
            continue;
        }

        usable_data.push((words, start_time));
    }

    Ok(usable_data)
}   