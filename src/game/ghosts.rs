use std::sync::{
            Mutex,
            Arc
};
use super::{
        position::Position,
        console,
        maze
};

/// Structure to store (thread safe) list of ghosts 
pub struct Ghosts{
    list: Arc<Mutex<Vec<Position>>>,
}

impl Ghosts {
    /// Initialize a thread safe struct
    pub fn new(ghosts: Vec<Position>) -> Ghosts {
        Ghosts{
            list: Arc::new(Mutex::new(ghosts)),
        }
    }

    /// Get a thread safe clone of Ghosts
    pub fn get_clone(&self) -> Ghosts {
        Ghosts {
            list: Arc::clone(&self.list),
        }
    }

    /// Move all the ghosts of one position in an available direction
    /// (depending of the maze)
    pub fn move_ghosts(&self, maze: &maze::Maze) {
        let mut ghosts = self.list.lock().unwrap();

        for position in &mut ghosts.iter_mut() {
            let mut new_pos;
            loop {
                new_pos = maze.make_move(position, &console::Entry::generate_random());

                if new_pos != *position {
                    break
                }
            }
            *position = new_pos;
        }
    }

    /// Verifiy is the given position is on conflict with one of
    /// the ghosts
    pub fn check_collision(&self, position: &Position) -> bool {
        let ghosts = self.list.lock().unwrap();

        ghosts.iter().find(|&ghost| {
            *ghost == *position
        }).is_some()
    }

    /// Display all the ghosts in the console
    pub fn display_ghosts(&self) {
        let ghosts = self.list.lock().unwrap();
        ghosts.iter().for_each(|pos| {
             console::display_at_pos(&pos, 'G');
        });
    }
}