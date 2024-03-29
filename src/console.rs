use termios::*;
use rand_derive2::RandGen;
use std::{
    io,
    io::Read,
    process,
    os::unix::io::AsRawFd,
};

use super::game::position as game;

#[derive(PartialEq, RandGen)]
pub enum Entry {
    #[rand_derive(skip)]
    Esc,
    Up,
    Down,
    Left,
    Right,
    #[rand_derive(skip)]
    None
}

impl Default for Entry {
    fn default() -> Self { Entry::None }
}

pub struct ConsoleMode;

impl Drop for ConsoleMode {
    fn drop(&mut self) {
        cleanup()
            .unwrap_or_else(|err| {
                eprintln!("Unable to restore cooked mode: {}", err);
                process::exit(1);
            })
    }
}

pub fn initialize() -> io::Result<ConsoleMode> {
    let fd = io::stdin().as_raw_fd();
    let mut termios = Termios::from_fd(fd)?;
    termios.c_lflag &= !ECHO;
    termios.c_lflag &= !ICANON;
    tcsetattr(fd, TCSANOW, &termios)?;
    Ok(ConsoleMode {})
}

pub fn cleanup() -> io::Result<()> {
    let fd = io::stdin().as_raw_fd();
    let mut termios = Termios::from_fd(fd)?;
    termios.c_lflag &= ECHO;
    termios.c_lflag &= ICANON;
    tcsetattr(fd, TCSANOW, &termios)?;
    Ok(())
}


pub fn read_input() -> Entry {
    let handle_error = |_| {
        eprintln!("Error happen when reading stdin");
        process::exit(1)
    };

    let mut buffer = [0; 102];
    let cnt = io::stdin()
                    .read(&mut buffer)
                    .unwrap_or_else(handle_error);

    if cnt == 1 && buffer[0] == 0x1b{
        Entry::Esc
    }
    else if cnt >= 3 {
        if buffer[0] == 0x1b && buffer[1] == 0x5b {
            match buffer[2] {
                0x41 => Entry::Up,
                0x42 => Entry::Down,
                0x43 => Entry::Right,
                0x44 => Entry::Left,
                0x1b => Entry::Esc,
                _    => Entry::None
            }
        }
        else {
            Entry::None
        }
    }
    else {
        Entry::None
    }

}

pub mod ansi {
    pub fn clear_screen() {
        print!("\x1b[2J");
        move_cursor(0, 0)
    }

    pub fn move_cursor(row: usize, col: usize) {
        print!("\x1b[{};{}H", row + 1, col + 1)
    }
}

/// Display the character at the given position
/// Must be used for walls that use 2 console characters
pub fn display_at_pos(pos: &game::Position, character: char) {
    ansi::move_cursor(pos.get_row(), pos.get_col() + 1);
    print!("{}", graphic::display_sprit(character));
}

/// Display the character every two columns
/// Must be used for small sprit (other than the wall)
pub fn display_at_pos_small(pos: &game::Position, character: char) {
    ansi::move_cursor(pos.get_row(), pos.get_col() * 2);
    print!("{}", graphic::display_sprit(character));
}


pub mod graphic {
    pub fn display_sprit(character: char) -> &'static str {
        match character {
            '#' => "🧱",
            '.' => "▫️ ",
            'P' => "😃",
            'G' => "👻",
            '-' => "  ",
            'X' => "💊",
            ' ' => "  ",
            _   => "ERROR",
        }
    }
}