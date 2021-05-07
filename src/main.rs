use std::process;

use pacrust::maze::Maze;
use pacrust::{
    console,
    console::Entry
};
use pacrust::maze::Position;

fn main() {
    // initialize game
    pacrust::console::initialize()
        .unwrap_or_else(|err| {
            eprintln!("Unable to activate cbreak mode: {}", err);
            process::exit(1);
        });

    // load resources
    let file_name = String::from("maze01.txt");
    let mut maze = Maze::load_maze(&file_name).unwrap_or_else(|err| {
        eprintln!("Problem loading maze: {}", err);
        process::exit(1);
    });

    // game loop
    loop {
        // update screen
        maze.print_screen();

        // process input
        let input = console::read_input();

        // process movement
        maze.move_player(&input);
        maze.move_ghosts();
        // process collisions

        // check game over
        if let Entry::Esc = input {
            break
        }

        // repeat
    }
}
