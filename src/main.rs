use std::process;

use pacrust::Maze;

fn main() {
    // initialize game
    let file_name = String::from("maze01.txt");

    // load resources
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
