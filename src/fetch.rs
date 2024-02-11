use reqwest::{
    header::{HeaderValue, ACCEPT_LANGUAGE, USER_AGENT},
    Client,
};
use std::{error::Error, ops::Not, process::Command, thread};

use crate::response::{
    Album, AlbumResponse, Artist, ArtistResponse, OrangeResult, Song, SongResponse,
};

const USR_AGENT: &str =
    "Mozilla/5.0 (X11; U; Linux armv7l; en-US; rv:1.9.2a1pre) Gecko/20090322 Fennec/1.0b2pre";
const YT_URL: &str = "https://www.youtube.com";
const SEARCH_URL: &str = "https://pipedapi.kavin.rocks/search";

#[allow(dead_code)]
pub fn play_selection(selection: &str) {
    let selection = selection.to_owned();
    thread::spawn(move || {
        Command::new("mpv")
            .arg(&selection.trim())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .expect("Failed to execute command. Ensure mpv is installed.");
    });
}

#[tokio::main]
pub async fn get_song(search: &str) -> Result<Vec<OrangeResult>, Box<dyn Error>> {
    if search.is_empty() {
        return Ok(Vec::new());
    }
    let client = Client::new();

    let resp = client
        .get(SEARCH_URL)
        .query(&[("q", search)])
        .query(&[("filter", "music_songs")])
        .header(USER_AGENT, HeaderValue::from_str(USR_AGENT).unwrap())
        .header(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"))
        .send()
        .await?;

    let body = resp.text().await?;
    let mut level_search: SongResponse = serde_json::from_str(&body)?;

    // run loop x times to get more results
    for _ in 0..2 {
        let resp = client
            .get(SEARCH_URL)
            .query(&[("q", search)])
            .query(&[("filter", "music_songs")])
            .query(&[("nextpage", &level_search.nextpage)])
            .header(USER_AGENT, HeaderValue::from_str(USR_AGENT).unwrap())
            .header(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"))
            .send()
            .await?;

        let body = resp.text().await?;
        let response: SongResponse = serde_json::from_str(&body)?;
        level_search.items.extend(response.items);
    }

    let mut results: Vec<OrangeResult> = vec![];

    if level_search.items.is_empty() {
        return Ok(Vec::new());
    }

    let mut videos: Vec<Song> = vec![];

    // push the videos to the videos vector if the title is "stream"
    for video in level_search.items {
        if video.video_type.to_lowercase() == "stream" && video.is_short.is_none().not() {
            videos.push(video);
        }
    }

    if videos.is_empty() {
        return Ok(Vec::new());
    }

    let duration = |d: i32| -> String {
        let minutes = d / 60;
        let seconds = d % 60;
        if seconds < 10 && minutes > 10 {
            return format!("{}:0{}", minutes, seconds);
        }
        if seconds > 10 && minutes < 10 {
            return format!("0{}:{}", minutes, seconds);
        }
        format!("{}:{}", minutes, seconds)
    };

    for video in videos {
        let title = video.title.as_ref().unwrap().to_string().replace("//", "");
        let watch_id = video.url.to_string();
        let video_url = format!("{}{}", YT_URL, watch_id);
        let vid_duration = duration(video.duration.unwrap());
        let uploader = video.uploader_name.unwrap().to_string().replace("//", "");
        let is_verified = video.uploader_verified.unwrap();
        results.push(OrangeResult {
            title,
            url: video_url,
            duration: vid_duration,
            uploader,
            is_verified,
        });
    }

    Ok(results)
}

#[tokio::main]
pub async fn get_album(search: &str) -> Result<Vec<Album>, Box<dyn Error>> {
    if search.is_empty() {
        return Ok(Vec::new());
    }
    let client = Client::new();

    let resp = client
        .get(SEARCH_URL)
        .query(&[("q", search)])
        .query(&[("filter", "music_albums")])
        .header(USER_AGENT, HeaderValue::from_str(USR_AGENT).unwrap())
        .header(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"))
        .send()
        .await?;

    let body = resp.text().await?;
    let album_data: AlbumResponse = serde_json::from_str(&body)?;

    let results: Vec<Album> = album_data.items;
    let mut album_results: Vec<Album> = vec![];

    if results.is_empty() {
        return Ok(Vec::new());
    }

    for album in results {
        let album_name = album.name.replace("//", "");
        let uploader = album.uploader_name.replace("//", "");
        let album_url = album.url;
        album_results.push(Album {
            name: album_name + " by " + uploader.as_str(),
            uploader_name: uploader,
            url: album_url,
        });
    }

    Ok(album_results)
}

#[tokio::main]
pub async fn get_artist(search: &str) -> Result<Vec<Artist>, Box<dyn Error>> {
    if search.is_empty() {
        return Ok(Vec::new());
    }
    let client = Client::new();

    let resp = client
        .get(SEARCH_URL)
        .query(&[("q", search)])
        .query(&[("filter", "music_artists")])
        .header(USER_AGENT, HeaderValue::from_str(USR_AGENT).unwrap())
        .header(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"))
        .send()
        .await?;

    let body = resp.text().await?;
    let artist_data: ArtistResponse = serde_json::from_str(&body)?;

    let results: Vec<Artist> = artist_data.items;

    if results.is_empty() {
        return Ok(Vec::new());
    }

    Ok(results)
}
