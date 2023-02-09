use std::io;
use std::io::Write;

use crate::models::VideoResource;

mod models;

fn main() {
    run();
}

fn run() {
    let quit_commands = ["q", "quit", "exit"];

    loop {
        let input = ask_input();

        if quit_commands.iter().any(|&cmd| cmd == input.to_lowercase()) {
            println!("Quitting...");
            return;
        }

        let video = VideoResource::build(input);
        match video {
            Some(v) => println!("Got a {} resource!", v.resource_type),
            None => {
                println!("Failed to get a video resource.")
            }
        }
    }
}

fn ask_input() -> String {
    let mut guess = String::new();

    while guess.trim().is_empty() {
        print!("■ Enter a YouTube URL or a command: "); // To support other platforms later.
        io::stdout().flush().unwrap(); // Necessary to display the previous line immediately.
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read from stdin");
    }

    guess.trim().to_string()
}
