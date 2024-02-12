use crate::{
    app::App,
    fetch::{get_album, get_album_tracks, get_artist, get_song},
};
use crossterm::event::{self, Event, KeyCode};
use std::io::{Result, Write};
use std::os::unix::net::UnixStream;

pub fn handle_key_event(app: &mut App) -> Result<bool> {
    if let Event::Key(event) = event::read()? {
        match event.code {
            KeyCode::Tab => {
                app.next();
            }
            KeyCode::Enter => match app.active_block {
                0 => {
                    app.active_block = 1;
                    app.is_search_mode = false;
                    app.search_query = app.search_input.clone();
                    app.search_input.clear();
                    app.song_data = get_song(&app.search_query).unwrap();
                    app.album_data = get_album(&app.search_query).unwrap();
                    app.artist_data = get_artist(&app.search_query).unwrap();
                    app.album_state.select(Some(0));
                    app.artist_state.select(Some(0));
                    app.song_state.select(Some(0));
                    app.from_album_block = false;
                }
                1 => {
                    if let Some(selected) = app.artist_state.selected() {
                        app.selected_artist = Some(app.artist_data[selected].clone());
                        app.active_block = 3;
                    }
                    app.song_data = get_song(&app.selected_artist.clone().unwrap().name).unwrap();
                    app.album_data =
                        get_album(&app.selected_artist.as_ref().unwrap().name).unwrap();
                    app.track_block_title = app.selected_artist.clone().unwrap().name;
                    app.song_state.select(Some(0));
                    app.from_album_block = false;
                }
                2 => {
                    if let Some(selected) = app.album_state.selected() {
                        app.selected_album = Some(app.album_data[selected].clone());
                        app.active_block = 3;
                    }
                    app.song_data =
                        get_album_tracks(&app.selected_album.as_ref().unwrap().url).unwrap();
                    app.track_block_title = app.selected_album.clone().unwrap().name;
                    app.song_state.select(Some(0));
                    app.from_album_block = true;
                }
                3 => {
                    if let Some(selected) = app.song_state.selected() {
                        app.selected_song = Some(app.song_data[selected].clone());
                    }
                    app.status_text = format!(
                        "Playing: {} by {}",
                        app.selected_song.clone().unwrap().title,
                        app.selected_song.clone().unwrap().uploader
                    );
                    if app.selected_song.is_some() {
                        if let Some(mpv) = &mut app.mpv {
                            let _ = mpv.kill();
                        }
                        let url = format!(
                            "{}{}",
                            app.base_url,
                            &app.selected_song.clone().unwrap().url
                        );
                        app.mpv = Some(
                            std::process::Command::new("mpv")
                                .arg(url)
                                .arg("--no-video")
                                .arg("--force-window=no")
                                .arg("--start=0")
                                .arg("--input-ipc-server=/tmp/mpvsocket")
                                .stdout(std::process::Stdio::null())
                                .stderr(std::process::Stdio::null())
                                .spawn()
                                .expect("mpv failed to start"),
                        );
                        app.is_playing = true;
                    }
                }
                _ => {}
            },

            // list navigation
            KeyCode::Up => match app.active_block {
                1 => {
                    if let Some(selected) = app.artist_state.selected() {
                        if selected > 0 {
                            app.artist_state.select(Some(selected - 1));
                        }
                    }
                }
                2 => {
                    if let Some(selected) = app.album_state.selected() {
                        if selected > 0 {
                            app.album_state.select(Some(selected - 1));
                        }
                    }
                }
                3 => {
                    if let Some(selected) = app.song_state.selected() {
                        if selected > 0 {
                            app.song_state.select(Some(selected - 1));
                        }
                    }
                }
                _ => {}
            },
            KeyCode::Down => match app.active_block {
                1 => {
                    if let Some(selected) = app.artist_state.selected() {
                        if selected < app.artist_data.len() - 1 {
                            app.artist_state.select(Some(selected + 1));
                        }
                    }
                }
                2 => {
                    if let Some(selected) = app.album_state.selected() {
                        if selected < app.album_data.len() - 1 {
                            app.album_state.select(Some(selected + 1));
                        }
                    }
                }
                3 => {
                    if let Some(selected) = app.song_state.selected() {
                        if selected < app.song_data.len() - 1 {
                            app.song_state.select(Some(selected + 1));
                        }
                    }
                }
                _ => {}
            },
            KeyCode::Esc => {
                if app.active_block == 0 {
                    app.active_block = 1;
                }
                if let Some(mpv) = &mut app.mpv {
                    let _ = mpv.kill();
                }
                app.is_playing = false;
                app.selected_song = None;
                app.status_text = String::new();
            }
            KeyCode::Char('+') => {
                if app.is_playing && app.volume + 5 <= 100 {
                    let mut stream = UnixStream::connect("/tmp/mpvsocket").unwrap();
                    write!(stream, "{{\"command\":[\"add\", \"volume\", \"5\"]}}\n").unwrap();
                    app.volume += 5;
                }
            }
            KeyCode::Char('-') => {
                if app.is_playing && app.volume - 5 >= 0 {
                    let mut stream = UnixStream::connect("/tmp/mpvsocket").unwrap();
                    write!(stream, "{{\"command\":[\"add\", \"volume\", \"-5\"]}}\n").unwrap();
                    app.volume -= 5;
                }
            }
            //for search
            KeyCode::Char('/') => {
                app.active_block = 0;
                app.search_cursor_position = app.search_input.len();
            }
            KeyCode::Char(c) if app.active_block == 0 => {
                app.search_input.insert(app.search_cursor_position, c);
                app.search_cursor_position += 1;
            }
            KeyCode::Char(' ') => {
                if app.active_block != 0 {
                    if app.is_playing {
                        let mut stream = UnixStream::connect("/tmp/mpvsocket").unwrap();
                        write!(stream, "{{\"command\":[\"cycle\", \"pause\"]}}\n").unwrap();
                        app.is_playing = true;
                    }
                }
            }
            KeyCode::Char('l') => {
                if app.is_playing {
                    let mut stream = UnixStream::connect("/tmp/mpvsocket").unwrap();
                    if app.looping {
                        write!(
                            stream,
                            "{{\"command\":[\"set_property\", \"loop-file\", \"no\"]}}\n"
                        )
                        .unwrap();
                        app.looping = false;
                    } else {
                        write!(
                            stream,
                            "{{\"command\":[\"set_property\", \"loop-file\", \"inf\"]}}\n"
                        )
                        .unwrap();
                        app.looping = true;
                    }
                }
            }
            KeyCode::Backspace if app.active_block == 0 && app.search_cursor_position > 0 => {
                app.search_input.remove(app.search_cursor_position - 1);
                app.search_cursor_position -= 1;
            }
            KeyCode::Left if app.active_block == 0 && app.search_cursor_position > 0 => {
                app.search_cursor_position -= 1;
            }
            KeyCode::Right
                if app.active_block == 0 && app.search_cursor_position < app.search_input.len() =>
            {
                app.search_cursor_position += 1;
            }

            KeyCode::Char('q') => {
                if let Some(mpv) = &mut app.mpv {
                    let _ = mpv.kill();
                }
                app.is_playing = false;
                return Ok(false);
            }
            _ => {}
        }
    }
    Ok(true)
}
