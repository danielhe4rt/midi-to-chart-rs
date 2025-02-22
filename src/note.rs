use ratatui::style::Color;

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Lane {
    Green,
    Red,
    Yellow,
    Blue,
    Orange,
}

impl Lane {
    pub fn get_color(&self) -> Color {
        match self {
            Lane::Green => Color::Green,
            Lane::Red => Color::Red,
            Lane::Yellow => Color::Yellow,
            Lane::Blue => Color::Blue,
            Lane::Orange => Color::Rgb(255, 165, 0),
        }
    }
    pub fn get_x(&self) -> f64 {
        match self {
            Lane::Green => 0.0,
            Lane::Red => 1.0,
            Lane::Yellow => 2.0,
            Lane::Blue => 3.0,
            Lane::Orange => 4.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Note {
    pub lane: Lane,
    pub time: f64,
}

pub fn get_notes() -> Vec<Note> {


    vec![
        Note {
            lane: Lane::Green,
            time: 1.0,
        },
        Note {
            lane: Lane::Red,
            time: 2.0,
        },
        Note {
            lane: Lane::Yellow,
            time: 3.0,
        },
        Note {
            lane: Lane::Blue,
            time: 4.0,
        },
        Note {
            lane: Lane::Orange,
            time: 5.0,
        },
        Note {
            lane: Lane::Orange,
            time: 6.0,
        },
        Note {
            lane: Lane::Green,
            time: 7.0,
        },
        Note {
            lane: Lane::Blue,
            time: 8.0,
        },
        Note {
            lane: Lane::Yellow,
            time: 9.0,
        },
        Note {
            lane: Lane::Red,
            time: 10.0,
        },
        Note {
            lane: Lane::Green,
            time: 11.0,
        },
        Note {
            lane: Lane::Red,
            time: 12.0,
        },
        Note {
            lane: Lane::Yellow,
            time: 13.0,
        },
        Note {
            lane: Lane::Blue,
            time: 14.0,
        },
        Note {
            lane: Lane::Orange,
            time: 15.0,
        },

    ]
}