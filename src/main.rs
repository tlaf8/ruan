use std::{io, time::Duration};
use anyhow::Result;
use ratatui::{backend::CrosstermBackend, Terminal};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Position {
    x: i16,
    y: i16,
    direction: Direction,
    trail: Vec<(i16, i16, char)>,
}

fn main() -> Result<()> {
    enable_raw_mode()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    terminal.clear()?;

    let mut pos = Position {
        y: 10,
        x: 10,
        direction: Direction::Right,
        trail: Vec::new(),
    };
    
    loop {
        terminal.draw(|frame| {
            let area = frame.area();
            frame.render_widget("Hello", area);
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,

                    KeyCode::Char('w') => pos.direction = Direction::Up,
                    KeyCode::Char('s') => pos.direction = Direction::Down,
                    KeyCode::Char('a') => pos.direction = Direction::Left,
                    KeyCode::Char('d') => pos.direction = Direction::Right,

                    KeyCode::Char(c) => {
                        pos.trail.push((pos.x, pos.y, c));

                        match pos.direction {
                            Direction::Up => pos.y -= 1,
                            Direction::Down => pos.y += 1,
                            Direction::Left => pos.x -= 1,
                            Direction::Right => pos.x += 1,
                        }
                    }

                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    Ok(())
}
