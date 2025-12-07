use std::time::Duration;

use clap::Parser;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::DefaultTerminal;
use selectors::{widgets::SnowField, App, Config};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    println!();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let config = Config::parse();
    let mut app = App::new(config);
    let mut snowfield = SnowField::new(200, 100, 600);

    loop {
        terminal.draw(|frame| {
            let area = frame.area();
            snowfield.set_size(area.width, area.height);

            // render snow first (background)
            snowfield.render(area, frame.buffer_mut());

            app.draw(frame);
        })?;

        // Update the snowfield
        snowfield.tick();

        // handle keyboard input without blocking
        if event::poll(Duration::from_millis(50))? {
            // 50 ms timeout
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()), // quit
                    KeyCode::Char(' ') => app.handle_spacebar(),
                    _ => {}
                }
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
