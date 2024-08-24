mod line;
mod telegram;
mod download;
mod utils;
mod token;

#[tokio::main]
async fn main() {
    let bot_token = token::get_bot_token().await;

    loop {
        println!("Select the platform to download stickers from (Line/Telegram):");
        let mut platform_choice = String::new();
        std::io::stdin().read_line(&mut platform_choice).expect("Failed to read input");

        match platform_choice.trim().to_lowercase().as_str() {
            "line" => {
                line::download_line_stickers().await;
            }
            "telegram" => {
                println!("Enter the Telegram sticker set name:");
                let mut sticker_set_name = String::new();
                std::io::stdin().read_line(&mut sticker_set_name).expect("Failed to read input");

                telegram::download_telegram_stickers(&bot_token, sticker_set_name.trim()).await;
            }
            _ => {
                println!("Invalid choice. Please enter 'Line' or 'Telegram'.");
            }
        }

        println!("Do you want to download another sticker pack? (yes/no):");
        let mut another = String::new();
        std::io::stdin().read_line(&mut another).expect("Failed to read input");

        if another.trim().to_lowercase() != "yes" {
            break;
        }
    }
}

