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
    let maze = Maze::load_maze(&file_name).unwrap_or_else(|err| {
        eprintln!("Problem loading maze: {}", err);
        process::exit(1);
    });

    let mut cnt = 0;

    // game loop
    loop {
        // update screen
        maze.print_screen();

        // process input
        let input = console::read_input();

        if let Entry::Esc = input {
            break
        }

        // process movement
        match input {
            Entry::Up => println!("up"),
            _ => println!("Other")
        };

        // process collisions

        // check game over

        // repeat
        cnt += 1;
        println!("{}", cnt);
    }
}
