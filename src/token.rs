use std::fs;
use std::io::{self};
use std::path::Path;

// Make the function public and async
pub async fn get_bot_token() -> String {
    const TOKEN_FILE: &str = "bot_token.txt";

    // Check if the token file exists
    if Path::new(TOKEN_FILE).exists() {
        // Read the token from the file asynchronously
        return fs::read_to_string(TOKEN_FILE)
            .expect("Failed to read bot token from file")
            .trim()
            .to_string();
    } else {
        // Prompt the user to enter the bot token
        println!("Enter your Telegram bot token:");
        let mut bot_token = String::new();
        io::stdin().read_line(&mut bot_token).expect("Failed to read bot token");

        // Save the token to a file
        let trimmed_token = bot_token.trim();
        fs::write(TOKEN_FILE, trimmed_token).expect("Failed to save bot token to file");

        return trimmed_token.to_string();
    }
}
