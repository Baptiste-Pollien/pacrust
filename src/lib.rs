use std::fs;
use std::fmt;
use std::{
    io::{BufRead, BufReader, Error as IoError},
    path::Path,
};

pub type IoRes<T> = Result<T, IoError>;

pub struct Maze{
    map: Vec<String>,
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.map {
            for character in line.chars() {
                let new_char = match character {
                    '#' => '▉',
                    '.' => '◦',
                    'P' => '☺',
                    'G' => '☠',
                    '-' => '▔',
                    'X' => 'X',
                    _   => ' ',
                };
                write!(f, "{}", new_char)?;
            }
            write!(f, "\n")?;
        }

        fmt::Result::Ok(())
    }
}

impl Maze {
    pub fn load_maze(file: impl AsRef<Path>) -> IoRes<Maze> {
        let mut file = fs::File::open(file)?;
        let mut reader = BufReader::new(&mut file);

        let mut map = Vec::with_capacity(15);
        let mut target;
        let mut bytes_read;

        'all_lines: loop {
            target = String::new();
            bytes_read = reader.read_line(&mut target)?;
            if bytes_read > 0 {
                map.push(target);
            } else {
                break 'all_lines;
            }
        }

        map.shrink_to_fit();
        Ok(Maze {map})

    }

    pub fn print_screen(&self) {
        println!("{}", self);
    }
}
