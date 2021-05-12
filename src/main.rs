use std::process;
use structopt::StructOpt;

use pacrust::{
    console,
    console::Entry,
    maze::Maze,
};


/// PacMan game coded in rust.
/// Based on https://github.com/danicat/pacgo
#[derive(StructOpt)]
#[structopt(name="PacRust")]
struct Cli {
    /// Enable graphical user interface
    #[structopt(long, short)]
    graphic: bool,

    /// Specify the input file for the maze
    #[structopt(long="--maze-file",
                short="-m",
                default_value="maze01.txt")]
    maze_file: String,
}

// use piston_window::*;

fn main() {
    // Read command line arguments
    let args = Cli::from_args();

    if args.graphic {
        println!("[WARNING] GUI not available for now...")
    }

    // initialize game
    pacrust::console::initialize()
        .unwrap_or_else(|err| {
            eprintln!("Unable to activate cbreak mode: {}", err);
            process::exit(1);
        });

    // load resources
    let file_name = String::from(args.maze_file);
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

    // End of the game
    console::ansi::clear_screen();
    if maze.get_nb_gums() == 0 {
        println!("You won!");
    }
    else if maze.get_lives() == 0 {
        println!("You died...");
    }
}


