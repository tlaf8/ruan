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
            let center_x = area.width as i32 / 2;
            let center_y = area.height as i32 / 2;

            for node in &pos.trail {
                let screen_x = node.x as i32 - pos.x as i32 + center_x;
                let screen_y = node.y as i32 - pos.y as i32 + center_y;

                if screen_x >= 0
                    && screen_y >= 0
                    && screen_x < area.width as i32
                    && screen_y < area.height as i32
                {
                    let x = screen_x as u16;
                    let y = screen_y as u16;
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

            buffer[(center_x as u16, center_y as u16)].set_char('_');
        })?;
    }

    disable_raw_mode()?;
    Ok(())
}
