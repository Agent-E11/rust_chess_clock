use std::{io::{stdout, Write}, error::Error};
use crossterm::event::{self, Event, KeyCode};

// Models the current state of the program
pub struct ProgramState {
    controls: Controls,
    running: bool,
    active_player: ActivePlayer,
    menu_active: bool,
}

// Contains all the possible controls and their `KeyCode`s
pub struct Controls {
    toggle_menu: KeyCode,
    toggle_start: KeyCode,
    toggle_settings: KeyCode,
    reset_time: KeyCode,
    switch_time: KeyCode,
    quit: KeyCode,
}

// Represents the current active player
#[derive(PartialEq, Debug)]
pub enum ActivePlayer {
    Player1,
    Player2,
}

impl ActivePlayer {
    // Toggle between `Player1` and `Player2`
    pub fn switch_player(&mut self) {
        *self = match self {
            ActivePlayer::Player1 => ActivePlayer::Player2,
            ActivePlayer::Player2 => ActivePlayer::Player1,
        }
    }
}

// Handles the program process
pub fn run() -> Result<(), Box<dyn Error>> {
    crossterm::terminal::enable_raw_mode().expect("oh no");
    let mut stdout = stdout();

    // Declare the default controls
    let default_controls = Controls {
        toggle_menu: KeyCode::Esc,
        toggle_start: KeyCode::Char('s'),
        toggle_settings: KeyCode::Char('s'),
        reset_time: KeyCode::Char('r'),
        switch_time: KeyCode::Char(' '),
        quit: KeyCode::Char('q'),
    };

    // Declare the default program state
    let mut program_state = ProgramState {
        controls: default_controls,
        running: false,
        active_player: ActivePlayer::Player1,
        menu_active: false,
    };

    loop {
        // Wait for a key event
        match event::read().unwrap() {
            Event::Key(key) => {
                // If it's not a key *press*, skip it
                if key.kind != crossterm::event::KeyEventKind::Press { continue; }

                // If the key is one of the keys in `program_state.controls`, perform the associated action
                match key.code {
                    c if c == program_state.controls.quit => break,
                    c if c == program_state.controls.toggle_menu => print!("Toggle menu\n\r"),
                    c if c == program_state.controls.toggle_start => print!("Toggle start\n\r"),
                    c if c == program_state.controls.reset_time => print!("Reset time\n\r"),
                    c if c == program_state.controls.switch_time => print!("Switch time\n\r"),
                    c if c == program_state.controls.toggle_settings => print!("Toggle settings\n\r"),
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

// ---------- Tests ----------
#[cfg(test)]
mod tests {
    use super::*;

    // Test the `switch_player()` method associated with the `ActivePlayer` enum
    #[test]
    fn switch_active_player() {
        let mut active_player = ActivePlayer::Player1;
        active_player.switch_player();
        assert_eq!(ActivePlayer::Player2, active_player);
        active_player.switch_player();
        assert_eq!(ActivePlayer::Player1, active_player);
    }
}