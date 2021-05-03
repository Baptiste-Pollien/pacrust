use std::{
    io::{Error as IoError,
         ErrorKind},
    path::Path,
    fmt,
    fs,
    ops::{Add, Sub},
};

use super::console;

pub type IoRes<T> = Result<T, IoError>;

enum Tile {
    Wall,
    Space
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let new_char = match self {
            Tile::Wall => '█',
            _          => ' ',
        };

        write!(f, "{}", new_char)?;

        fmt::Result::Ok(())
    }
}

pub struct Position {
    row: usize,
    col: usize
}


impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {row: self.row + other.row, col: self.col + other.col}
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {row: self.row - other.row, col: self.row - other.row}
    }
}

struct Gum {
    pos:  Position,
    mega: bool
}

pub struct Maze {
    map:     Vec<Vec<Tile>>,
    pacman:  Position,
    ghosts:  Vec<Position>,
    gums:    Vec<Gum>,
    width:   usize,
    heigth:  usize,
    berserk: usize
}

impl Maze {
    pub fn load_maze(file: impl AsRef<Path>) -> IoRes<Maze> {
        let contents                     = fs::read_to_string(file)?;
        let mut ghosts: Vec<Position>    = Vec::new();
        let mut gums: Vec<Gum>           = Vec::new();
        let mut pacman: Option<Position> = None;

        let map: Vec<Vec<Tile>> = contents
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line
                    .bytes()
                    .enumerate()
                    .map(|(col, c)| {
                        match c {
                            b'#' => Tile::Wall,
                            b'P' => {
                                pacman = Some(Position{row, col});
                                Tile::Space
                            },
                            b'G' => {
                                ghosts.push(Position{row, col});
                                Tile::Space
                            }
                            b'.' => {
                                gums.push(Gum {
                                    pos: Position{row, col}, 
                                    mega:false
                                });
                                Tile::Space
                            }
                            b'X' => {
                                gums.push(Gum {
                                    pos: Position{row, col}, 
                                    mega:true
                                });
                                Tile::Space
                            }
                            _ => Tile::Space
                        }
                    }).collect()
            }).collect();

        let heigth = map.len();
        let width = map[0].len();

        if let Some(pacman) = pacman {
            Ok(Maze {
                map,
                pacman,
                ghosts,
                gums,
                width,
                heigth,
                berserk: 0
            })
        }
        else {
            Err(IoError::new(ErrorKind::Other, 
                "No player found"))
        }

    }

    pub fn print_screen(&self) {
        //console::ansi::clear_screen();
        // Display maze
        for line in &self.map {
            for c in line {
                print!("{}", c)
            }
            println!("")
        }
    }
}


// impl fmt::Display for Maze {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         for line in &self.map {
//             for character in line.chars() {
//                 let new_char = match character {
//                     '#' => '█',
//                     '.' => '◦',
//                     'P' => '☺',
//                     'G' => '☠',
//                     '-' => '▔',
//                     'X' => '♥',
//                     _   => ' ',
//                 };
//                 write!(f, "{}", new_char)?;
//             }
//             write!(f, "\n")?;
//         }

//         fmt::Result::Ok(())
//     }
// }