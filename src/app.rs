use crate::draw::draw_lanes;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{DefaultTerminal, Frame};

#[derive(Debug, Default)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub speed: f64,
    pub should_draw: bool,
    pub mouse_x: i32,
    pub mouse_y: i32,
    pub lanes_count: i32,
    pub time: i32,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self {
            running: true,
            speed: 1.0,
            should_draw: true,
            mouse_x: 0,
            mouse_y: 0,
            lanes_count: 4,
            time: 0,
        }
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        terminal.draw(|frame| self.draw(frame))?;
        self.should_draw = false;
        while self.running {
            self.handle_crossterm_events()?;

            if self.should_draw {
                terminal.draw(|frame| self.draw(frame))?;
            }
        }
        Ok(())
    }

    /// Renders the user interface.
    ///
    /// This is where you add new widgets. See the following resources for more information:
    /// - <https://docs.rs/ratatui/latest/ratatui/widgets/index.html>
    /// - <https://github.com/ratatui/ratatui/tree/master/examples>
    fn draw(&mut self, frame: &mut Frame) {
        // Drawing State manager = DON'T RETURN ANYTHING, JUST LET IT DRAW
        draw_lanes(self, frame);
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(mouse) => {
                // Handle mouse events here.
                self.mouse_x = mouse.row as i32;
                self.mouse_y = mouse.column as i32;
                self.should_draw = true;
            }
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Char('d')) => {
                if self.time >= 0 {
                    self.time += 1;
                    self.should_draw = true;
                }
            }
            (_, KeyCode::Char('f')) => {
                if self.time > 0 {
                    self.time -= 1;
                    self.should_draw = true;
                }
            }

            (_, KeyCode::Char('a')) => {
                // max 2.0
                if self.speed >= 0.1 && self.speed < 2.0 {
                    self.speed += 0.1;
                    self.should_draw = true;
                }
            }
            (_, KeyCode::Char('z')) => {
                // max 2.0
                if self.lanes_count == 4 {
                    self.lanes_count = 5;
                    self.should_draw = true;
                } else {
                    self.lanes_count = 4;
                    self.should_draw = true;
                }
            }
            (_, KeyCode::Char('s')) => {
                // min 0.1
                if self.speed >= 0.2 && self.speed as i32 <= 2 {
                    self.speed -= 0.1;
                    self.should_draw = true;
                }
            }
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            // Add other key handlers here.
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}
