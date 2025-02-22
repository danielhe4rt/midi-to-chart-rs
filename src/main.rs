use std::io::stdout;
use crossterm::event::EnableMouseCapture;
use crossterm::execute;
pub use app::App;

pub mod app;
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
