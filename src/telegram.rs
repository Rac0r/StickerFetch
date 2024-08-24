// src/telegram.rs

use reqwest::Client;
use serde::Deserialize;
use std::path::PathBuf;
use crate::download::save_image;
use crate::utils::sanitize_and_create_folder;

#[derive(Debug, Deserialize)]
pub struct TelegramStickerSetResponse {
    pub ok: bool,
    pub result: TelegramStickerSet,
}

#[derive(Debug, Deserialize)]
pub struct TelegramStickerSet {
    pub name: String,
    pub title: String,
    pub stickers: Vec<TelegramSticker>,
}

#[derive(Debug, Deserialize)]
pub struct TelegramSticker {
    pub file_id: String,
}

#[derive(Debug, Deserialize)]
struct FileResponse {
    result: File,
}

#[derive(Debug, Deserialize)]
struct File {
    file_id: String,
    file_path: String,
}

pub async fn download_telegram_stickers(bot_token: &str, set_name: &str) {
    let client = reqwest::Client::new();
    let url = format!(
        "https://api.telegram.org/bot{}/getStickerSet?name={}",
        bot_token, set_name
    );

    let response = client.get(&url).send().await.expect("Failed to fetch sticker set");

    // Parse the JSON response
    let sticker_set_response: TelegramStickerSetResponse = response.json().await
        .expect("Failed to parse JSON");

    if !sticker_set_response.ok {
        panic!("Sticker set not found or invalid set name.");
    }

    let sticker_set = sticker_set_response.result;
    let pack_name = sanitize_and_create_folder(&sticker_set.title);

    for sticker in sticker_set.stickers {
        let file_response = get_file_info(&client, bot_token, &sticker.file_id).await;
        let file_url = format!("https://api.telegram.org/file/bot{}/{}", bot_token, file_response.result.file_path);
        let path = PathBuf::from(format!("{}/{}.png", pack_name, sticker.file_id));
        save_image(&file_url, &path).await;
    }
}

async fn get_file_info(client: &Client, bot_token: &str, file_id: &str) -> FileResponse {
    let url = format!(
        "https://api.telegram.org/bot{}/getFile?file_id={}",
        bot_token, file_id
    );

    let response = client.get(&url).send().await.expect("Failed to get file info");
    response.json().await.expect("Failed to parse file info JSON")
}
