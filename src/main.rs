// Gemini 2.5 Pro was used to create this code

// Declare the modules that Rust will look for in other files
mod airport;
mod game;
mod plane;
mod flight_plan;

use std::io::{self, Write};
use std::thread;
use std::time::Duration;

// --- MAIN FUNCTION ---
// Entry point, contains the game loop
fn main() {
    // We can use game::Game because we declared `mod game` above
    let mut game = game::Game::new();
    let tick_rate = Duration::from_millis(2000); // Game updates every 2 seconds

    println!("--- Air Traffic Controller Simulator ---");
    println!("Welcome! Manage the airport. Don't mess up.");

    while !game.is_game_over() {
        // 1. Draw the game state
        game.draw();

        // 2. Get user input (with a simple prompt)
        print!("> ");
        io::stdout().flush().unwrap(); // Make sure "> " appears before input
        
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input!");
            continue;
        }

        // 3. Process the command
        game.process_command(input.trim().to_string());

        // 4. Update the game state (the "tick")
        game.update();
        
        // 5. Wait for the next tick
        thread::sleep(tick_rate);
    }
}
