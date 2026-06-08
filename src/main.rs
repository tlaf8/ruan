use std::{io, time::Duration};
use anyhow::Result;
use ratatui::{backend::CrosstermBackend, Terminal};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::style::{Color, Modifier, Style};

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct TrailItem {
    x: i16,
    y: i16,
    ch: char,
    dir: Direction,
    highlighted: bool,
}

struct Position {
    x: i16,
    y: i16,
    dir: Direction,
    typed: String,
    trail: Vec<TrailItem>,
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

                    KeyCode::Backspace => {
                        if let Some(node) = pos.trail.pop() {
                            pos.x = node.x;
                            pos.y = node.y;
                            pos.typed.pop();

                            pos.dir = match pos.trail.last() {
                                Some(node) => node.dir,
                                None => Direction::Right,
                            };
                        }
                    }

                    KeyCode::Char(c) => {
                        if !c.is_ascii_lowercase() {
                            continue;
                        }

                        pos.typed.push(c);

                        let highlighted_word_len = if pos.typed.ends_with("up") {
                            pos.dir = Direction::Up;
                            Some(2)
                        } else if pos.typed.ends_with("down") {
                            pos.dir = Direction::Down;
                            Some(4)
                        } else if pos.typed.ends_with("left") {
                            pos.dir = Direction::Left;
                            Some(4)
                        } else if pos.typed.ends_with("right") {
                            pos.dir = Direction::Right;
                            Some(5)
                        } else {
                            None
                        };

                        pos.trail.push(TrailItem {
                            x: pos.x,
                            y: pos.y,
                            ch: c,
                            dir: pos.dir,
                            highlighted: false,
                        });

                        if let Some(word_len) = highlighted_word_len {
                            let start = pos.trail.len().saturating_sub(word_len);

                            for trail_item in &mut pos.trail[start..] {
                                trail_item.highlighted = true;
                            }
                        }

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

            for node in &pos.trail {
                if node.x >= 0 && node.y >= 0 {
                    let x = node.x as u16;
                    let y = node.y as u16;

                    if x < area.width && y < area.height {
                        let cell = &mut buffer[(x, y)];
                        cell.set_char(node.ch);

                        if node.highlighted {
                            cell.set_style(
                                Style::default()
                                    .fg(Color::Yellow)
                                    .add_modifier(Modifier::BOLD),
                            );
                        }
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
