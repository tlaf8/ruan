#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct TrailItem {
    pub x: i16,
    pub y: i16,
    pub ch: char,
    pub dir: Direction,
    pub highlighted: bool,
}

pub struct Position {
    pub x: i16,
    pub y: i16,
    pub dir: Direction,
    pub typed: String,
    pub trail: Vec<TrailItem>,
}

impl Position {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            dir: Direction::Right,
            typed: String::new(),
            trail: Vec::new(),
        }
    }
}
