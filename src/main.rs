use std::process;

use pacrust::maze::Maze;
use pacrust::{
    console,
    console::Entry
};

use piston_window::*;

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
        maze.process_collisions();

        // check game over
        if Entry::Esc == input 
           || maze.get_nb_gums() == 0 
           || maze.get_lives() == 0 {
            break
        }

        // repeat
    }

    maze.print_screen();
    if maze.get_nb_gums() == 0 {
        println!("You win !");
    }
    else if maze.get_lives() == 0 {
        println!("You die...");
    }
}


