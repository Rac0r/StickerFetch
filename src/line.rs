// src/line.rs

use crate::download::{download_stickers, download_static_stickers};
use crate::utils::sanitize_and_create_folder;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct StickerPack {
    title: Title,
    stickers: Vec<Sticker>,
}

#[derive(Debug, Deserialize)]
struct Title {
    en: String,
}

#[derive(Debug, Deserialize)]
pub struct Sticker {
    pub id: u32,
}

pub async fn download_line_stickers() {
    println!("Enter the Sticker Pack ID:");
    let mut pack_id_input = String::new();
    std::io::stdin().read_line(&mut pack_id_input).expect("Failed to read line");
    let pack_id: u32 = pack_id_input.trim().parse().expect("Please enter a valid number");

    let pack_meta = fetch_pack_metadata(pack_id).await;
    let pack_name = sanitize_and_create_folder(&pack_meta.title.en);

    let contains_animated = check_for_animated_stickers(pack_id, &pack_meta.stickers).await;

    if contains_animated {
        println!("Select the type of files you want to download: png, gif, or both");
        let mut file_type = String::new();
        std::io::stdin().read_line(&mut file_type).expect("Failed to read line");
        let file_type = file_type.trim().to_lowercase();

        download_stickers(pack_id, &pack_meta.stickers, &pack_name, &file_type).await;
    } else {
        println!("This pack contains only PNG stickers. Downloading PNGs...");
        download_static_stickers(&pack_meta.stickers, &pack_name).await;
    }
}

async fn fetch_pack_metadata(pack_id: u32) -> StickerPack {
    let url = format!(
        "https://dl.stickershop.line.naver.jp/products/0/0/1/{}/android/productInfo.meta",
        pack_id
    );

    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .expect("Failed to build client");

    let response = client.get(&url).send().await.expect("Failed to fetch metadata");

    if response.status().is_success() {
        response.json().await.expect("Failed to parse JSON")
    } else {
        eprintln!("Error fetching metadata for pack ID: {}", pack_id);
        std::process::exit(1);
    }
}

async fn check_for_animated_stickers(pack_id: u32, stickers: &[Sticker]) -> bool {
    for sticker in stickers {
        let url = format!(
            "https://sdl-stickershop.line.naver.jp/products/0/0/1/{}/iphone/animation/{}@2x.png",
            pack_id, sticker.id
        );
        let response = reqwest::get(&url).await;
        if response.is_ok() && response.unwrap().status().is_success() {
            return true;
        }
    }
    false
}
