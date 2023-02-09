use std::io;
use std::io::Write;

use crate::models::*;

mod models;

fn main() {
    let quit_commands = ["q", "quit", "exit"];

    loop {
        let input = ask_input();
        // let input = "https://www.youtube.com/watch?v=5B1rB894B1U".to_string();
        // let input = "https://www.youtube.com/playlist?list=PLnijyl7KNnKEkYBvzeSsP5fhOSHJCM8sX".to_string();
        // let input = "https://www.youtube.com/channel/UCHd3WDMgMl12t0aJCdZrmJA/playlists".to_string();

        if quit_commands.iter().any(|&cmd| cmd == input.to_lowercase()) {
            println!("Quitting...");
            return;
        }

        let video = match VideoResource::build(input) {
            Ok(v) => v,
            Err(e) => {
                println!("Failed to get a video resource: {}", e);
                continue;
            },
        };

        match download_video(video) {
            Ok(_) => println!("Download complete!"),
            Err(e) => println!("Could not download: {}", e.message)
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
    println!("Will download {} at {}", video.resource_type, video.full_resource_uri());
    Ok(())
}
