use std::{
    io::{Error as IoError,
         ErrorKind},
    path::Path,
    fs,
};
use super::console;

pub type IoRes<T> = Result<T, IoError>;

/// Structure and functions to interact with the position of elements of
/// the game.
pub mod position;
use self::position::Position;

/// Enum for static elements of the maze
mod tile;
pub use self::tile::Tile;

struct Gum {
    pos:  Position,
    mega: bool
}

pub struct Game {
    maze:     Vec<Vec<Tile>>,
    pacman:  Position,
    ghosts:  Vec<Position>,
    gums:    Vec<Gum>,
    width:   usize,
    heigth:  usize,
    berserk: usize,
    lives: u8,
    max_gums: usize,
}

impl Game {
    pub fn load_maze(file: impl AsRef<Path>) -> IoRes<Game> {
        let contents                     = fs::read_to_string(file)?;
        let mut ghosts: Vec<Position>    = Vec::new();
        let mut gums: Vec<Gum>           = Vec::new();
        let mut pacman: Option<Position> = None;

        let maze: Vec<Vec<Tile>> = contents
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

        let heigth   = maze.len();
        let width    = maze[0].len();
        let max_gums = gums.len();

        if let Some(pacman) = pacman {
            Ok(Game {
                maze,
                pacman,
                ghosts,
                gums,
                width,
                heigth,
                berserk: 0,
                lives: 1,
                max_gums,
            })
        }
        else {
            Err(IoError::new(ErrorKind::Other, 
                "No player found"))
        }

    }

    fn get_tile(&self, row:usize, col:usize) -> Option<&Tile> {
        if row < self.heigth && col < self.width {
            Some(&self.maze[row][col])
        }
        else {
            None
        }
    }

    pub fn get_nb_gums(&self) -> usize {
        self.gums.len()
    }

    pub fn get_lives(&self) -> u8 {
        self.lives
    }

    pub fn get_max_gums(&self) -> usize {
        self.max_gums
    }

    pub fn get_score(&self) -> usize {
        self.max_gums as usize - self.gums.len()
    }

    fn make_move(&self, 
                 current_pos: &Position,
                 dir: &console::Entry
    ) -> Position {
        let mut pos = Position::new(current_pos.row, current_pos.col);

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
            console::Entry::Right => {
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

        if let Some(tile) = self.get_tile(pos.row, pos.col) {
            if *tile == Tile::Wall {
                pos.row = current_pos.row;
                pos.col = current_pos.col;
            }
        }

        pos
    }

    pub fn move_player(&mut self, dir: &console::Entry) {
        let player = self.make_move(&self.pacman, &dir);

        self.pacman = player;

        for i in 0..self.gums.len() {
            if self.gums[i].pos == self.pacman {
                self.gums.remove(i);
                break;
            }
        }
    }

    pub fn move_ghosts(&mut self) {
        self.ghosts = self.ghosts
                          .iter()
                          .map(|pos| -> Position {
            let mut new_pos;
            loop {
                new_pos = self.make_move(pos, &console::Entry::generate_random());

                if new_pos != *pos {
                    break
                }
            }
            new_pos
        }).collect();
    }

    pub fn process_collisions(&mut self) {
        if let Some(_) = self.ghosts.iter().find(|&ghost| {
            *ghost == self.pacman
        }){
            self.lives = self.lives - 1;
        }
    }

    pub fn print_screen(&self) {
        console::ansi::clear_screen();
        // Display maze
        for line in &self.maze {
            for c in line {
                print!("{}", c)
            }
            println!("")
        }

        // Display gums
        self.gums.iter().for_each(|gum| {
            let c = if gum.mega {'X'} else {'.'};
            console::display_at_pos(&gum.pos, c);
        });

        // Display ghosts
        self.ghosts.iter().for_each(|pos| {
            console::display_at_pos(&pos, 'G');
        });

        // Display player
        console::display_at_pos(&self.pacman, 'P');

        // Reset cursor
        console::ansi::move_cursor(self.heigth-1, self.width);

        // Print score
        println!("\nScore: {} \tLives: {}", self.get_score(), self.lives);
    }
}
