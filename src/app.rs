use crate::note::{get_notes, Lane};
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::prelude::Color;
use ratatui::widgets::canvas::{Canvas, Circle, Context, Line};
use ratatui::{widgets::Block, DefaultTerminal, Frame};

#[derive(Debug, Default)]
pub struct App {
    /// Is the application running?
    running: bool,
    speed: f64,
    should_draw: bool,
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
        // Get the size of the frame
        let width = frame.area().width;
        let height = frame.area().height;

        // Set the bounds of the canvas
        let left = 0.0;
        let right = f64::from(width);
        let bottom = 0.0;
        let top = f64::from(height);

        // Get the base lane height
        let base_lane_count = self.lanes_count;
        let base_lane_spacing = if base_lane_count == 5 { 4.7 } else { 3.90 };
        let base_lane_height = base_lane_count as f64 * self.speed;

        let base_lane_note_x = (width as f64 / base_lane_count as f64);

        let mut notes = get_notes();
        let canvas = Canvas::default()
            .block(Block::bordered().title("Canvas"))
            .x_bounds([left, right])
            .y_bounds([bottom, top])
            .paint(|ctx| {
                // Draw Info
                draw_info(self, ctx, height.clone() as i32, width.clone() as i32);

                ctx.layer();
                for lanes in 0..(base_lane_count + 1) {
                    let line_width = base_lane_note_x + (lanes as f64 * width as f64 / 10.0);
                    // Draw Lanes
                    ctx.draw(&Line::new(
                        line_width,
                        1.0,
                        line_width,
                        (height as f64),
                        Color::White,
                    ));
                }

                ctx.layer();
                for lane_spacing in (0..height).step_by(base_lane_height as usize) {
                    let line_width =
                        base_lane_note_x + (base_lane_count as f64 * width as f64 / 10.0);

                    // Draw Lanes
                    ctx.draw(&Line::new(
                        (width as f64 / base_lane_spacing), // TODO: get a better value to this resizing
                        base_lane_height + lane_spacing as f64,
                        line_width as f64 - 1.0,
                        base_lane_height + lane_spacing as f64,
                        Color::White,
                    ));
                }
                ctx.layer();

                // Draw Notes (this is a guitar hero chart viewer)

                for note in &notes[self.time as usize..] {
                    let note_lane = base_lane_note_x + (note.lane.get_x() * width as f64 / 10.0);

                    if base_lane_count == 4 && note.lane == Lane::Orange {
                        continue;
                    }
                    ctx.draw(&Circle {
                        x: note_lane + (base_lane_note_x / base_lane_count as f64) - self.speed,
                        y: (note.time as f64 * base_lane_height ) - (2.5) - (self.time as f64 * base_lane_spacing ),
                        radius: 1.0,
                        color: note.lane.get_color(),
                    });
                }
            });

        frame.render_widget(canvas, frame.area());
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

                if self.time >= 0  {
                    self.time += 1;
                    self.should_draw = true;
                }
            }
            (_, KeyCode::Char('f')) => {

                if self.time > 0  {
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

fn draw_info(app: &App, ctx: &mut Context, height: i32, width: i32) {
    ctx.print(80.0, height as f64 - 1.0, format!("Speed: {}", app.speed));
    ctx.print(
        80.0,
        height as f64 - 2.0,
        format!("Mouse X: {}", app.mouse_x),
    );
    ctx.print(
        80.0,
        height as f64 - 3.0,
        format!("Mouse Y: {}", app.mouse_y),
    );

    ctx.print(
        80.0,
        height as f64 - 4.0,
        format!("Lanes: {}", app.lanes_count),
    );
    ctx.print(
        80.0,
        height as f64 - 5.0,
        format!("Time: {}", app.lanes_count),
    );

    // THERE'S NO ZERO = ZERO = ONE
    ctx.print(2.0, 0.0, "Y");
    ctx.print(1.0, 1.0, "X");
    ctx.print(2.0, 1.0, "0");
    // Draw Grid
    for i in 1..height {
        ctx.print(2.0, i as f64 + 1.0, format!("{}", i));
    }

    for i in 1..width {
        ctx.print(i as f64 + 2.0, 1.0, format!("{}", i));
    }
}
