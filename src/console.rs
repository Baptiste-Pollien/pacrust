use std::process;
use termios::*;
use std::io;
use std::os::unix::io::AsRawFd;
use std::io::Read;
use rand::Rng;

use super::maze;

pub enum Entry {Esc, Up, Down, Left, Rigth, None}

impl Entry {
    pub fn rand_dir() -> Entry {
        let rand_number = rand::thread_rng().gen_range(1..5);

        match rand_number {
            1 => Entry::Up,
            2 => Entry::Down,
            3 => Entry::Rigth,
            _ => Entry::Left
        }
    }
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

    let mut buffer = [0; 3];
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
                0x43 => Entry::Rigth,
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

pub fn display_at_pos(pos: &maze::Position, character: char) {
    ansi::move_cursor(pos.get_row(), pos.get_col());
    print!("{}", graphic::display_sprit(character));
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

pub mod graphic {
    pub fn display_sprit(character: char) -> char {
        match character {
            '#' => '█',
            '.' => '◦',
            'P' => '☺',
            'G' => '☠',
            '-' => '▔',
            'X' => '♥',
            _   => character,
        }
    }
}