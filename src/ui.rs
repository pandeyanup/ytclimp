use crate::{app::App, blocks, key_event};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{CrosstermBackend, Terminal},
    style::{Color, Style},
    text::Span,
    widgets::{List, ListItem, Paragraph, Wrap},
};
use std::io;

pub fn run_ui(mut app: App) -> io::Result<()> {
    enable_raw_mode()?;

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;
    // Select the first item in each list
    app.album_state.select(Some(0));
    app.artist_state.select(Some(0));
    app.song_state.select(Some(0));

    loop {
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(8),
                        Constraint::Percentage(if app.is_playing { 82 } else { 84 }),
                        Constraint::Percentage(if app.is_playing { 10 } else { 8 }),
                    ]
                    .as_ref(),
                )
                .split(frame.size());

            let middle_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(35), Constraint::Percentage(65)].as_ref())
                .split(chunks[1]);

            let middle_sub_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(middle_chunks[0]);

            let blocks = [
                blocks::search_block(app.active_block),
                blocks::artist_block(app.active_block),
                blocks::album_block(app.active_block),
                blocks::track_block(app.active_block),
                blocks::status_block(app.active_block),
            ];

            // search_block
            let seearch_block = blocks[0].to_owned().title("Search");
            let mut search_display = if app.active_block == 0 {
                app.search_input.clone()
            } else {
                "What do you want to search?".to_string()
            };
            if app.active_block == 0
                && app.search_cursor_position <= search_display.len()
                && search_display.is_char_boundary(app.search_cursor_position)
            {
                search_display.insert(app.search_cursor_position, '|');
            }
            let search_widget = Paragraph::new(search_display.as_ref() as &str)
                .block(seearch_block)
                .wrap(Wrap { trim: true });

            // artist_block
            let artist_block = blocks[1].to_owned().title("Artists");
            let artist_list = List::new(app.artist_data.iter().map(|artist| {
                ListItem::new(artist.name.clone())
                    .style(Style::default().fg(Color::Rgb(137, 220, 235)))
            }))
            .block(artist_block)
            .highlight_style(Style::default().bg(Color::White).fg(Color::Black));
            frame.render_stateful_widget(artist_list, middle_sub_chunks[0], &mut app.artist_state);

            // album_block
            let album_block = blocks[2].to_owned().title("Albums");
            let album_list = List::new(app.album_data.iter().map(|album| {
                ListItem::new(album.name.clone())
                    .style(Style::default().fg(Color::Rgb(238, 212, 159)))
            }))
            .block(album_block)
            .highlight_style(Style::default().bg(Color::White).fg(Color::Black));
            frame.render_stateful_widget(album_list, middle_sub_chunks[1], &mut app.album_state);

            // track_block
            let track_block = blocks[3].to_owned().title(app.track_block_title.clone());
                let song_list = List::new(app.song_data.iter().enumerate().map(|(index, song)| {
                    ListItem::new(format!("{}. {} by 『{}』", index+1, song.title, if app.from_album_block {
                        app.selected_album.clone().unwrap().uploader_name
                    } else {
                    song.uploader.clone()
                    }))
                        .style(Style::default().fg(Color::Rgb(198, 160, 246)))
                }))
                .block(track_block)
                .highlight_style(Style::default().bg(Color::White).fg(Color::Black));

            frame.render_stateful_widget(song_list, middle_chunks[1], &mut app.song_state);

            // status_block
            let status_block = blocks[3].to_owned().title("Status");
            let playing_styled = if let Some(selected_song) = &app.selected_song {
                Span::styled(
                    format!("Playing: {} by 『{}』", selected_song.title.clone(), selected_song.uploader.clone()),
                    Style::default().fg(Color::Rgb(203, 166, 247)),
                )
            } else {
                Span::styled(
                    format!("'q' - quit | 'space' - pause/play | 'l' - loop playing | '+' or '-' - change volume | 'esc' - stop playing"),
                    Style::default().fg(Color::Rgb(180, 190, 254)),
                )
            };
            let statusbar = Span::styled(
                format!("Vol: {} | Loop: {} ", app.volume, app.looping.to_string()),
                Style::default().fg(Color::Green),
            )
            .to_right_aligned_line();
            let status_widget = List::new(vec![
                ListItem::new(playing_styled),
                ListItem::new(statusbar),
            ])
            .block(status_block);

            // Render the widgets
            frame.render_widget(search_widget, chunks[0]);
            frame.render_widget(status_widget, chunks[2]);
        })?;

        if !key_event::handle_key_event(&mut app)? {
            break;
        }
    }

    disable_raw_mode()?;
    Ok(())
}
