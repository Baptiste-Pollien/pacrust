use std::fmt;
use super::console::graphic;

/// Possible static elements of the maze
#[derive(PartialEq, Debug)]
pub enum Tile {
    Wall,
    Space
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let new_char = match self {
            Tile::Wall => '#',
            _          => ' ',
        };

        write!(f, "{}", graphic::display_sprit(new_char))?;

        fmt::Result::Ok(())
    }
}

impl Default for Tile {
    fn default() -> Self { Tile::Wall }
}