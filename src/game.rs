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

/// Module to manage ghost in a different thread
mod ghosts;
pub use self::ghosts::Ghosts;

/// Module to manage Maze in different thread
mod maze;
pub use self::maze::Maze;

struct Gum {
    pos:  Position,
    mega: bool
}

pub struct Game {
    maze:    maze::Maze,
    pacman:  Position,
    ghosts:  Ghosts,
    gums:    Vec<Gum>,
    width:   usize,
    heigth:  usize,
    berserk: usize,
    lives: u8,
    max_gums: usize,
}

impl Game {
    /// Load the maze from the given path and return an initialized 
    /// Game 
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
                maze: Maze::new(maze),
                pacman,
                ghosts: Ghosts::new(ghosts),
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

    /// Return the number of gums left
    pub fn get_nb_gums(&self) -> usize {
        self.gums.len()
    }

    /// Return the number of lives left
    pub fn get_lives(&self) -> u8 {
        self.lives
    }

    /// Return the number of game available at the start
    pub fn get_max_gums(&self) -> usize {
        self.max_gums
    }

    /// Return the score of the game
    /// (the number of gum eaten)
    pub fn get_score(&self) -> usize {
        self.max_gums as usize - self.gums.len()
    }

    /// Get a thread safe clone for the ghosts
    pub fn get_clone_ghosts(&self) -> ghosts::Ghosts {
        self.ghosts.get_clone()
    }

    /// Get a thread safe clone for the maze
    pub fn get_clone_maze(&self) -> Maze {
        self.maze.get_clone()
    }

    /// Move the player for the given direction
    pub fn move_player(&mut self, dir: &console::Entry) {
        let player = self.maze.make_move(&self.pacman, &dir);

        self.pacman = player;

        for i in 0..self.gums.len() {
            if self.gums[i].pos == self.pacman {
                self.gums.remove(i);
                break;
            }
        }
    }

    /// Verify if there is any collision between the player
    /// and the ghosts. In case of a collision, a live is taken.
    pub fn process_collisions(&mut self) {
        if self.ghosts.check_collision(&self.pacman) {
            self.lives = self.lives - 1;
        }
    }

    /// Display all the game
    pub fn print_screen(&self) {
        console::ansi::clear_screen();
        // Display maze
        self.maze.display_maze();
        console::ansi::move_cursor(1, 1);

        // Display gums
        self.gums.iter().for_each(|gum| {
            let c = if gum.mega {'X'} else {'.'};
            console::display_at_pos_small(&gum.pos, c);
        });

        // Display ghosts
        self.ghosts.display_ghosts();

        // Display player
        console::display_at_pos_small(&self.pacman, 'P');

        // Reset cursor
        console::ansi::move_cursor(self.heigth-1, self.width);

        // Print score
        println!("\nScore: {} \tLives: {}", self.get_score(), self.lives);
    }
}
