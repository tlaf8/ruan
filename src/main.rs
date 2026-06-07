use std::{io, time::Duration};
use anyhow::Result;
use ratatui::{backend::CrosstermBackend, Terminal};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
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
    dir: Direction,
    typed: String,
    trail: Vec<(i16, i16, char)>,
}

fn main() -> Result<()> {
    enable_raw_mode()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    terminal.clear()?;

    let mut pos = Position {
        y: 0,
        x: 0,
        dir: Direction::Right,
        typed: String::new(),
        trail: Vec::new(),
    };

    loop {
        // Handle input events
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match key.code {
                    KeyCode::Char('q') => break,

                    KeyCode::Char(c) => {
                        if !c.is_ascii_lowercase() {
                            continue;
                        }

                        pos.typed.push(c);

                        if pos.typed.ends_with("up") {
                            pos.dir = Direction::Up;
                        } else if pos.typed.ends_with("down") {
                            pos.dir = Direction::Down;
                        } else if pos.typed.ends_with("left") {
                            pos.dir = Direction::Left;
                        } else if pos.typed.ends_with("right") {
                            pos.dir = Direction::Right;
                        }

                        pos.trail.push((pos.x, pos.y, c));

                        match pos.dir {
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

        // Draw the map
        terminal.draw(|frame| {
            let area = frame.area();
            let buffer = frame.buffer_mut();

            for &(x, y, c) in &pos.trail {
                if x >= 0 && y >= 0 {
                    let x = x as u16;
                    let y = y as u16;

                    if x < area.width && y < area.height {
                        buffer[(x, y)].set_char(c);
                    }
                }
            }

            if pos.x >= 0 && pos.y >= 0 {
                let x = pos.x as u16;
                let y = pos.y as u16;

                if x < area.width && y < area.height {
                    match pos.dir {
                        Direction::Up => { buffer[(x, y)].set_char('_'); }, 
                        Direction::Down => { buffer[(x, y)].set_char('_'); }, 
                        Direction::Left => { buffer[(x, y)].set_char('_'); }, 
                        Direction::Right => { buffer[(x, y)].set_char('_'); }, 
                    }
                }
            }
        })?;
    }

    disable_raw_mode()?;
    Ok(())
}
