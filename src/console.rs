use std::process;
use termios::*;
use std::io;
use std::os::unix::io::AsRawFd;
use std::io::Read;
use std::io::Write;

pub enum Entry {Esc, Up, Down, Left, Rigth, None}

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

pub mod ansi {
    pub fn clear_screen() {
        print!("\x1b[2J");
        move_cursor(0, 0)
    }

    pub fn move_cursor(row: usize, col: usize) {
        print!("\x1b[{};{}H", row + 1, col + 1)
    }
}