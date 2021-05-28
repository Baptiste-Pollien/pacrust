use std::sync::Arc;

use super::{
    Tile,
    position::Position,
    console,
};


/// Thread safe representation for the maze
pub struct Maze {
    map: Arc<Vec<Vec<Tile>>>
}

impl Maze {
    /// Create a new thread safe maze
    pub fn new(map: Vec<Vec<Tile>>) -> Maze {
        Maze {
            map: Arc::new(map),
        }
    }

    /// Get a thread safe clone for the maze
    pub fn get_clone(&self) -> Maze{
        Maze {
            map: Arc::clone(&self.map),
        }
    }

    /// Return a couple containing the heigh and the
    /// width of the maze
    fn get_heigth_width(&self) -> (usize, usize) {
        (self.map.len(), self.map[0].len())
    }

    /// Get the tile in the maze for a given coordinate
    pub fn get_tile(&self, row:usize, col:usize) -> Option<&Tile> {
        let (heigth, width)   = self.get_heigth_width();

        if row < heigth && col < width {
            Some(&self.map[row][col])
        }
        else {
            None
        }
    }

    /// Create a new position from the current position 
    /// and the given direction. The new position is unchanged
    /// in the case where there is a wall.
    pub fn make_move(&self, 
                    current_pos: &Position,
                    dir: &console::Entry
    ) -> Position {
        let (heigth, width) = self.get_heigth_width();

        let mut pos = Position::new(current_pos.row, current_pos.col);

        match dir {
            console::Entry::Up => {
                if pos.row == 0 {
                    pos.row = heigth - 1;
                }
                else {
                    pos.row = pos.row - 1;
                }
            },
            console::Entry::Down => {
                if pos.row == heigth - 1 {
                    pos.row = 0;
                }
                else {
                    pos.row = pos.row + 1;
                }
            },
            console::Entry::Right => {
                if pos.col == width - 1 {
                    pos.col = 0;
                }
                else {
                    pos.col = pos.col + 1;
                }
            },
            console::Entry::Left => {
                if pos.col == 0 {
                    pos.col = width - 1;
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

    /// Display maze in the console
    pub fn display_maze(&self) {
        for line in self.map.iter() {
            for c in line {
                print!("{}", c)
            }
            println!("")
        }
    }
}
