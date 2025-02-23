pub use app::App;
use crossterm::event::EnableMouseCapture;
use crossterm::execute;
use std::io::stdout;

pub mod app;
mod draw;
mod lanes;
mod note;

fn main() -> color_eyre::Result<()> {
    print!("\x1B[2J\x1B[1;1H");
    color_eyre::install()?;
    execute!(stdout(), EnableMouseCapture)?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}
