#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct TrailNode {
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
    pub trail: Vec<TrailNode>,
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

    pub fn pop_single(&mut self) {
        if let Some(node) = self.trail.pop() {
            self.x = node.x;
            self.y = node.y;
            self.typed.pop();

            self.dir = match self.trail.last() {
                Some(node) => node.dir,
                None => Direction::Right,
            };
        }
    }

    pub fn push(&mut self, c: char) {
        self.typed.push(c);

        let direction_word_len = if self.typed.ends_with("up") {
            self.dir = Direction::Up;
            Some(2)
        } else if self.typed.ends_with("down") {
            self.dir = Direction::Down;
            Some(4)
        } else if self.typed.ends_with("left") {
            self.dir = Direction::Left;
            Some(4)
        } else if self.typed.ends_with("right") {
            self.dir = Direction::Right;
            Some(5)
        } else {
            None
        };

        self.trail.push(TrailNode {
            x: self.x,
            y: self.y,
            ch: c,
            dir: self.dir,
            highlighted: false,
        });

        if let Some(word_len) = direction_word_len {
            let start = self.trail.len().saturating_sub(word_len);

            for trail_item in &mut self.trail[start..] {
                trail_item.highlighted = true;
            }
        }

        match self.dir {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
}