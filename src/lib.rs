use std::{io::{stdout, Write}, error::Error};
use crossterm::event::{self, Event, KeyCode};


pub fn run() -> Result<(), Box<dyn Error>> {
    crossterm::terminal::enable_raw_mode().expect("oh no");
    let mut stdout = stdout();

    

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