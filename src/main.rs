use std::process;
use structopt::StructOpt;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use pacrust::{
    console,
    game::Game,
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
    let mut game = Game::load_maze(&file_name).unwrap_or_else(|err| {
        eprintln!("Problem loading maze: {}", err);
        process::exit(1);
    });

    // process input (async)
    let (tx, rx) = mpsc::sync_channel(0);
    thread::spawn(move || {
        loop {
            let input = console::read_input();

            if let Err(_) = tx.send(input) {
                break
            };
        }
    });

    // process for ghosts
    let ghosts = game.get_clone_ghosts();
    let maze   = game.get_clone_maze();
    thread::spawn(move || {
        // Waiting before starting moving ghosts
        thread::sleep(Duration::from_millis(50));
        loop {
            ghosts.move_ghosts(&maze);

            thread::sleep(Duration::from_millis(200));
        };
    });


    // game loop
    loop {
        // update screen
        game.print_screen();

        // process input
        let input = rx.try_recv().unwrap_or_default();

        // process movement
        game.move_player(&input);

        // process collisions
        game.process_collisions();

        // check game over
        if console::Entry::Esc == input 
           || game.get_nb_gums() == 0 
           || game.get_lives() == 0 {
            break
        }

        // repeat
        thread::sleep(Duration::from_millis(50));
    }

    // End of the game
    console::ansi::clear_screen();
    if game.get_nb_gums() == 0 {
        println!("You have won!");
    }
    else if game.get_lives() == 0 {
        println!("Game over... ({}/{})", 
                        game.get_score(), 
                        game.get_max_gums());
    }
}


