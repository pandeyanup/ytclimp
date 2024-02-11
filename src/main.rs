mod app;
mod blocks;
mod fetch;
mod key_event;
mod response;
mod ui;

use app::App;
use std::io;

fn main() -> io::Result<()> {
    let app = App::new();
    ui::run_ui(app)
}
