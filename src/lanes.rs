use ratatui::prelude::Color;
use ratatui::widgets::canvas::{Circle, Context, Line, Map, Painter, Shape};
use crate::App;
use crate::note::{Lane, Note};

pub struct Lanes {
    pub lanes_count: i32,
    pub time: f64,
    pub speed: f64,
    pub width: u16,
    pub height: u16,
    pub notes: Vec<Note>,
}

impl Lanes {
    pub fn draw(&self, ctx: &mut Context) {
        let base_lane_count = self.lanes_count;
        let base_lane_spacing = if base_lane_count == 5 { 4.7 } else { 3.90 };
        let base_lane_height = base_lane_count as f64 * self.speed;
        let base_lane_note_x = (self.width as f64 / base_lane_count as f64);

        for lanes in 0..(base_lane_count + 1) {
            let line_width = base_lane_note_x + (lanes as f64 * self.width as f64 / 10.0);
            // Draw Lanes
            ctx.draw(&Line::new(
                line_width,
                1.0,
                line_width,
                (self.height as f64),
                Color::White,
            ));
        }

        ctx.layer();
        for lane_spacing in (0..self.height).step_by(base_lane_height as usize) {
            let line_width =
                base_lane_note_x + (base_lane_count as f64 * self.width as f64 / 10.0);

            // Draw Lanes
            ctx.draw(&Line::new(
                (self.width as f64 / base_lane_spacing), // TODO: get a better value to this resizing
                base_lane_height + lane_spacing as f64,
                line_width as f64 - 1.0,
                base_lane_height + lane_spacing as f64,
                Color::White,
            ));
        }
        ctx.layer();

        // Draw Notes (this is a guitar hero chart viewer)

        for note in &self.notes[self.time as usize..] {
            let note_lane = base_lane_note_x + (note.lane.get_x() * self.width as f64 / 10.0);

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
    }
}

