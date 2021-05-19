use std::fmt;
use super::console::graphic;

#[derive(PartialEq, Debug)]
pub enum Tile {
    Wall,
    Space
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let new_char = match self {
            Tile::Wall => graphic::display_sprit('#'),
            _          => graphic::display_sprit(' '),
        };

        write!(f, "{}", new_char)?;

        fmt::Result::Ok(())
    }
}

impl Default for Tile {
    fn default() -> Self { Tile::Wall }
}