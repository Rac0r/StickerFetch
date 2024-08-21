mod metadata;
mod download;
mod conversion;
mod utils;

use std::io;
use metadata::fetch_pack_metadata;
use download::{check_for_animated_stickers, download_stickers, download_static_stickers};
use utils::sanitize_and_create_folder;

#[tokio::main]
async fn main() {
    loop {
        println!("Enter the Sticker Pack ID:");
        let mut pack_id_input = String::new();
        io::stdin().read_line(&mut pack_id_input).expect("Failed to read line");
        let pack_id: u32 = pack_id_input.trim().parse().expect("Please enter a valid number");

        let pack_meta = fetch_pack_metadata(pack_id).await;
        let pack_name = sanitize_and_create_folder(&pack_meta.title.en);

        let contains_animated = check_for_animated_stickers(pack_id, &pack_meta.stickers).await;

        if contains_animated {
            // Ask user for desired file type if animations are present
            println!("Select the type of files you want to download: png, gif, or both");
            let mut file_type = String::new();
            io::stdin().read_line(&mut file_type).expect("Failed to read line");
            let file_type = file_type.trim().to_lowercase();

            download_stickers(pack_id, &pack_meta.stickers, &pack_name, &file_type).await;
        } else {
            // If only PNGs are present, skip file type choice
            println!("This pack contains only PNG stickers. Downloading PNGs...");
            download_static_stickers(&pack_meta.stickers, &pack_name).await;
        }

        // Ask if user wants to download another sticker pack
        println!("Do you want to download another sticker pack? (yes/no)");
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("Failed to read line");
        if answer.trim().eq_ignore_ascii_case("no") {
            break;
        }
    }
}