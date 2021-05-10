use std::process;

use pacrust::maze::Maze;
use pacrust::{
    console,
    console::Entry
};

use structopt::StructOpt;

/// PacMan game coded in rust.
/// From https://github.com/danicat/pacgo
#[derive(StructOpt)]
#[structopt(name="PacRust")]
struct Cli {
    /// Enable graphical user interface
    #[structopt(long, short)]
    graphic: bool,

    /// Display the game in the console
    #[structopt(long, short)]
    console: bool,
}


use piston_window::*;

fn main() {
    let args = Cli::from_args();

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


