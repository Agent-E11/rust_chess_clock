use std::{io::{stdout, Write}, error::Error};
use crossterm::event::{self, Event, KeyCode};

pub struct Controls {
    toggle_menu: KeyCode,
    toggle_start: KeyCode,
    toggle_settings: KeyCode,
    reset_time: KeyCode,
    switch_time: KeyCode,
    quit: KeyCode,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    crossterm::terminal::enable_raw_mode().expect("oh no");
    let mut stdout = stdout();

    let default_controls = Controls {
        toggle_menu: KeyCode::Esc,
        toggle_start: KeyCode::Char('s'),
        toggle_settings: KeyCode::Char('s'),
        reset_time: KeyCode::Char('r'),
        switch_time: KeyCode::Char(' '),
        quit: KeyCode::Char('q'),
    };

    loop {
        // Wait for a key event
        match event::read().unwrap() {
            
            Event::Key(key) => {
                // If it's not a key *press*, skip it
                if key.kind != crossterm::event::KeyEventKind::Press { continue; }

                match key.code {
                    qu if qu == default_controls.quit => break,
                    tm if tm == default_controls.toggle_menu => print!("Toggle menu\n\r"),
                    ts if ts == default_controls.toggle_start => print!("Toggle start\n\r"),
                    rt if rt == default_controls.reset_time => print!("Reset time\n\r"),
                    st if st == default_controls.switch_time => print!("Switch time\n\r"),
                    ts if ts == default_controls.toggle_settings => print!("Toggle settings\n\r"),
                    k => print!("{k:?}\n\r"),
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