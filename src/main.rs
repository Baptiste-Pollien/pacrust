use std::process;

use pacrust::maze::Maze;

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

    // game loop
    loop {
        // update screen
        maze.print_screen();

        // process input

        // process movement

        // process collisions

        // check game over

        // Temp: break infinite loop
        break

        // repeat
    }
}
