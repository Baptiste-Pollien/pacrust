use std::ops::{Add, Sub};

/// Structure to store the position on the map of an element.
/// Only the Game should modify a Position
#[derive(PartialEq, Debug)]
pub struct Position {
    pub (super) row: usize,
    pub (super) col: usize
}

impl Position {
    /// Create a new position
    pub fn new(row: usize, col: usize) -> Position {
        Position{row, col}
    }

    /// Return the row value of the position
    pub fn get_row(&self) -> usize {
        self.row
    }

    /// Return the col value of the position
    pub fn get_col(&self) -> usize {
        self.col
    }
}

/// Adds two positions.
///
/// # Examples
///
/// ```
/// # use pacrust::game::position::Position;
/// 
/// let pos1 = Position::new(1, 2);
/// let pos2 = Position::new(2, 1);
/// let res  = pos1 + pos2;
///
/// assert_eq!(res, Position::new(3, 3));
/// ```
impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {row: self.row + other.row, col: self.col + other.col}
    }
}

/// Substracts two positions.
///
/// # Examples
///
/// ```
/// # use pacrust::game::position::Position;
/// 
/// let pos1 = Position::new(2, 2);
/// let pos2 = Position::new(1, 1);
/// let res  = pos1 - pos2;
///
/// assert_eq!(res, Position::new(1, 1));
/// ```
impl Sub for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {row: self.row - other.row, col: self.row - other.row}
    }
}

#[cfg(test)]
mod tests {
    use super::Position;

    #[test]
    fn test_position_new() {
        let pos1 = Position::new(0, 0);
        let pos2 = Position{row: 0,
                            col: 0,};

        assert_eq!(pos1, pos2);
    }

    #[test]
    fn test_position_getter() {
        let pos = Position::new(1, 2);

        assert_eq!(pos.get_row(), 1);
        assert_eq!(pos.get_col(), 2);
    }

    #[test]
    fn test_position_add() {
        let pos1 = Position::new(1, 2);
        let pos2 = Position::new(2, 1);
        let res  = pos1 + pos2;

        assert_eq!(res, Position::new(3, 3));
    }

    #[test]
    fn test_position_sub() {
        let pos1 = Position::new(1, 1);
        let pos2 = Position::new(2, 2);
        let res  = pos2 - pos1;

        assert_eq!(res, Position::new(1, 1));
    }

    #[test]
    #[should_panic]
    fn test_position_sub_fail() {
        let pos1 = Position::new(2, 2);
        let pos2 = Position::new(1, 1);

        let _res = pos2 - pos1;
    }

}