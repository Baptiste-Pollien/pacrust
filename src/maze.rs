use std::{
    io::{Error as IoError,
         ErrorKind},
    path::Path,
    fmt,
    fs,
    ops::{Add, Sub},
};

use super::console;
use super::console::graphic::display_sprit;

pub type IoRes<T> = Result<T, IoError>;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Tile {
    Wall,
    Space
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let new_char = match self {
            Tile::Wall => display_sprit('#'),
            _          => display_sprit(' '),
        };

        write!(f, "{}", new_char)?;

        fmt::Result::Ok(())
    }
}

#[derive(Copy, Clone)]
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

impl Position {
    pub fn new(row: usize, col: usize) -> Position {
        Position{row, col}
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

    fn get_tile(&self, row:usize, col:usize) -> Tile {
        if row < self.heigth && col < self.width {
            self.map[row][col]
        }
        else {
            Tile::Wall
        }
    }

    fn make_move(&self, dir: console::Entry) -> Position {
        let mut pos = Position::new(self.pacman.row, self.pacman.col);

        match dir {
            console::Entry::Up => {
                if pos.row == 0 {
                    pos.row = self.heigth - 1;
                }
                else {
                    pos.row = pos.row - 1;
                }
            },
            console::Entry::Down => {
                if pos.row == self.heigth - 1 {
                    pos.row = 0;
                }
                else {
                    pos.row = pos.row + 1;
                }
            },
            console::Entry::Rigth => {
                if pos.col == self.width - 1 {
                    pos.col = 0;
                }
                else {
                    pos.col = pos.col + 1;
                }
            },
            console::Entry::Left => {
                if pos.col == 0 {
                    pos.col = self.width - 1;
                }
                else {
                    pos.col = pos.col - 1;
                }
            }
            _ => (),
        }
        if self.get_tile(pos.row, pos.col) == Tile::Wall {
            self.pacman
        }
        else {
            pos
        }
    }

    pub fn move_player(&mut self, dir: console::Entry) {
        self.pacman = self.make_move(dir);
    }

    pub fn print_screen(&self) {
        console::ansi::clear_screen();
        // Display maze
        for line in &self.map {
            for c in line {
                print!("{}", c)
            }
            println!("")
        }

        // Display player
        let pos_pac = &self.pacman;
        console::ansi::move_cursor(pos_pac.row, pos_pac.col);
        print!("{}", display_sprit('P'));

        //reset cursor
        console::ansi::move_cursor(self.heigth-1, self.width);
        println!("");
    }
}
