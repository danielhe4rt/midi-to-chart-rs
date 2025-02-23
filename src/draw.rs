use crate::lanes::Lanes;
use crate::note::get_notes;
use crate::App;
use ratatui::widgets::canvas::Canvas;
use ratatui::widgets::Block;
use ratatui::Frame;

pub fn draw_lanes(app: &mut App, frame: &mut Frame) {
    // Get the size of the frame
    let width = frame.area().width;
    let height = frame.area().height;

    // Set the bounds of the canvas
    let left = 0.0;
    let right = f64::from(width);
    let bottom = 0.0;
    let top = f64::from(height);

    let mut notes = get_notes();
    let canvas = Canvas::default()
        .block(Block::bordered().title("Canvas"))
        .x_bounds([left, right])
        .y_bounds([bottom, top])
        .paint(|ctx| {
            // Draw Info
            let lane = Lanes {
                lanes_count: app.lanes_count,
                time: app.time as f64,
                speed: app.speed,
                width,
                height,
                notes: notes.clone(),
            };
            lane.draw(ctx);
        });

    frame.render_widget(canvas, frame.area());
}
