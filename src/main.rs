use std::io;
use std::io::Write;

fn main() {
    run();
}

fn run() {
    let quit_commands = ["q", "quit", "exit"];

    loop {
        let input = ask_input();
        println!("Got \"{}\"!", input);

        if quit_commands.iter().any(|&cmd| cmd == input.to_lowercase()) {
            println!("Quitting...");
            return;
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