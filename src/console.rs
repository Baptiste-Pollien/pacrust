use std::process;
use termios::*;
use std::io;
use std::os::unix::io::AsRawFd;

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

pub mod ansi {
    pub fn clear_screen() {
        print!("\x1b[2J");
        move_cursor(0, 0)
    }

    pub fn move_cursor(row: usize, col: usize) {
        print!("\x1b[{};{}H", row + 1, col + 1)
    }
}