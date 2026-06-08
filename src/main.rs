use std::{io, time::Duration};
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind}, 
    terminal::{disable_raw_mode, enable_raw_mode}
};
use ratatui::{backend::CrosstermBackend, Terminal};
use ratatui::style::{Color, Modifier, Style};

mod trail;
use trail::{Position};

mod cursor;
use cursor::{Cursor};

fn main() -> Result<()> {
    enable_raw_mode()?;

    let mut cursor = Cursor::new();
    let mut pos = Position::new();
    let mut terminal = Terminal::new(
        CrosstermBackend::new(
            io::stdout()
        )
    )?;

    terminal.clear()?;

    loop {
        cursor.update();

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match key.code {
                    KeyCode::Char('q') => break,

                    KeyCode::Backspace => { 
                        pos.pop_single(); 
                    },

                    KeyCode::Char(c) => {
                        if !c.is_ascii_lowercase() {
                            continue;
                        }

                        pos.push(c);
                    },

                    _ => {},
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

            buffer[(center_x as u16, center_y as u16)]
                .set_char(if cursor.visible { '█' } else { ' ' });

        })?;
    }

    disable_raw_mode()?;
    Ok(())
}
