use std::io;
use std::io::Write;

use crate::video::*;

mod video;

fn main() {
    let (successes, failures) = run();

    println!(
        "Done with {} success(es) and {} failure(s).",
        successes, failures
    )
}

fn run() -> (u32, u32) {
    let quit_commands = ["q", "quit", "exit"];

    let mut successes: u32 = 0;
    let mut failures: u32 = 0;

    loop {
        let input = ask_input();

        if quit_commands.iter().any(|&cmd| cmd == input.to_lowercase()) {
            println!("\nQuitting...");
            return (successes, failures);
        }

        let video = match VideoResource::build(input) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to get a video resource: {}", e);
                continue;
            }
        };

        match download_video(video) {
            Ok(_) => {
                println!("Download complete!");
                successes += 1;
            }
            Err(e) => {
                eprintln!("Download failed: {}", e.message);
                failures += 1;
            }
        }
    }
}

fn ask_input() -> String {
    let mut guess = String::new();

    while guess.trim().is_empty() {
        print!("\n■ Enter a YouTube URL or a command: "); // To support other platforms later.
        io::stdout().flush().unwrap(); // Necessary to display the previous line immediately.
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read from stdin");
    }

    guess.trim().to_string()
}

fn download_video(video: VideoResource) -> Result<(), VideoDownloadError> {
    println!(
        "Will download {} at {}",
        video.resource_type,
        video.full_resource_uri()
    );
    Ok(())
}
