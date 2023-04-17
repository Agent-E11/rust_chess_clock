use std::{io::{stdout, Write}, error::Error};
use crossterm::event::{self, Event, KeyCode};

pub struct Controls {
    toggle_menu: KeyCode,
    toggle_start: KeyCode,
    reset_time: KeyCode,
    switch_time: KeyCode,
    quit: KeyCode,
    settings: KeyCode,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    crossterm::terminal::enable_raw_mode().expect("oh no");
    let mut stdout = stdout();

    let default_controls = Controls {
        toggle_menu: KeyCode::Esc,
        toggle_start: KeyCode::Char('s'),
        reset_time: KeyCode::Char('r'),
        switch_time: KeyCode::Char(' '),
        quit: KeyCode::Char('q'),
        settings: KeyCode::Char('s'),
    };

    loop {
        // Wait for a key event
        match event::read().unwrap() {
            
            Event::Key(key) => {
                // If it's not a key *press*, skip it
                if key.kind != crossterm::event::KeyEventKind::Press { continue; }

                match key.code {
                    KeyCode::Char('q') => break,
                    k => print!("{k:?}"),
                }
            }
            _ => (),
        }

        // Flush the output stream, ignore any error
        let _ = stdout.flush();
    }
    crossterm::terminal::disable_raw_mode().expect("oh no");
    
    Ok(())
}