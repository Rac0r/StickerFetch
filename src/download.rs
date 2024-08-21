use std::path::PathBuf;
use crate::utils::save_image;
use crate::conversion::convert_apng_to_gif;
use crate::metadata::Sticker;

use reqwest;

/// Checks if there are any animated stickers in the pack.
pub async fn check_for_animated_stickers(pack_id: u32, stickers: &[Sticker]) -> bool {
    for sticker in stickers {
        let url = format!(
            "https://sdl-stickershop.line.naver.jp/products/0/0/1/{}/iphone/animation/{}@2x.png",
            pack_id, sticker.id
        );
        let response = reqwest::get(&url).await;
        if response.is_ok() && response.unwrap().status().is_success() {
            return true; // Found at least one animated sticker
        }
    }
    false // No animated stickers found
}

/// Downloads stickers based on the selected file type (png, gif, or both).
pub async fn download_stickers(pack_id: u32, stickers: &[Sticker], pack_name: &str, pack_ext: &str) {
    match pack_ext {
        "both" => {
            download_static_stickers(stickers, pack_name).await;
            download_animated_stickers(pack_id, stickers, pack_name).await;
        }
        "png" => download_static_stickers(stickers, pack_name).await,
        "gif" => download_animated_stickers(pack_id, stickers, pack_name).await,
        _ => eprintln!("Invalid file type provided."),
    }
}

/// Downloads static PNG stickers.
pub async fn download_static_stickers(stickers: &[Sticker], pack_name: &str) {
    for sticker in stickers {
        let url = format!(
            "http://dl.stickershop.line.naver.jp/stickershop/v1/sticker/{}/iphone/sticker@2x.png",
            sticker.id
        );
        let path = PathBuf::from(format!("{}/{}.png", pack_name, sticker.id));
        save_image(&url, &path).await;
    }
}

/// Downloads animated stickers (converts APNG to GIF).
async fn download_animated_stickers(pack_id: u32, stickers: &[Sticker], pack_name: &str) {
    for sticker in stickers {
        let apng_url = format!(
            "https://sdl-stickershop.line.naver.jp/products/0/0/1/{}/iphone/animation/{}@2x.png",
            pack_id, sticker.id
        );
        let apng_path = PathBuf::from(format!("{}/{}.apng", pack_name, sticker.id));
        let gif_path = PathBuf::from(format!("{}/{}.gif", pack_name, sticker.id));

        save_image(&apng_url, &apng_path).await;
        convert_apng_to_gif(&apng_path, &gif_path).expect("Failed to convert APNG to GIF");

        // Remove APNG file after conversion to GIF
        tokio::fs::remove_file(&apng_path).await.expect("Failed to delete APNG file");
    }
}

