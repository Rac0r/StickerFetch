use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct StickerPack {
    pub title: Title,
    pub stickers: Vec<Sticker>,
}

#[derive(Debug, Deserialize)]
pub struct Title {
    pub en: String,
}

#[derive(Debug, Deserialize)]
pub struct Sticker {
    pub id: u32,
}

/// Fetches the metadata of a sticker pack from the given pack ID.
pub async fn fetch_pack_metadata(pack_id: u32) -> StickerPack {
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
