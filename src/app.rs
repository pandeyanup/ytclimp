use ratatui::widgets::ListState;

use crate::{
    fetch::{get_album, get_artist, get_song},
    response::{Album, Artist, OrangeResult},
};

pub struct App {
    pub active_block: usize,
    pub artist_data: Vec<Artist>,
    pub artist_state: ListState,
    pub selected_artist: Option<Artist>,
    pub song_data: Vec<OrangeResult>,
    pub selected_song: Option<OrangeResult>,
    pub song_state: ListState,
    pub album_data: Vec<Album>,
    pub selected_album: Option<Album>,
    pub album_state: ListState,
    pub search_input: String,
    pub search_query: String,
    pub is_search_mode: bool,
    pub search_cursor_position: usize,
    pub status_text: String,
    pub is_playing: bool,
    pub mpv: Option<std::process::Child>,
    pub volume: i32,
    pub looping: bool,
    pub track_block_title: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            active_block: 1,
            search_query: String::new(),
            search_input: String::new(),
            is_search_mode: false,
            artist_data: get_artist("Aimer milet").unwrap(),
            selected_artist: None,
            artist_state: ListState::default(),
            song_data: get_song("RADWIMPS milet").unwrap(),
            selected_song: None,
            song_state: ListState::default(),
            album_data: get_album("Aimer RADWIMPS").unwrap(),
            selected_album: None,
            album_state: ListState::default(),
            search_cursor_position: 0,
            status_text: String::new(),
            is_playing: false,
            mpv: None,
            volume: 100,
            looping: false,
            track_block_title: "Tracks".to_string(),
        }
    }

    pub fn next(&mut self) {
        // no go to status tab. Why go there?
        self.active_block = (self.active_block + 1) % 4;
    }
}
