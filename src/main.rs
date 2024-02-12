mod app;
mod blocks;
mod fetch;
mod key_event;
mod response;
mod ui;

use app::App;
use clap::{command, Arg};
use std::io;

fn main() -> io::Result<()> {
    let matches = command!()
        .about("A cli YT music player")
        .version("1.1.0")
        .arg(
            Arg::new("search")
                .short('s')
                .long("search")
                .help_heading("Search for the music")
                .required(false)
                .num_args(0),
        )
        .arg(
            Arg::new("piped")
                .short('p')
                .long("piped")
                .help_heading("Use piped instance for playing")
                .required(false)
                .num_args(0),
        )
        .get_matches();
    let search_mode = matches.get_one::<bool>("search").unwrap();
    let piped_mode = matches.get_one::<bool>("piped").unwrap();

    let mut search_data = String::new();
    if *search_mode {
        search_data = dialoguer::Input::new()
            .with_prompt("Search for a Music".to_string())
            .interact()
            .unwrap();
    }

    let app = App::new(&search_data.to_owned(), piped_mode.to_owned());
    ui::run_ui(app)?;
    Ok(())
}
