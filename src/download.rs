// src/download.rs

use reqwest;
use std::path::PathBuf;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use crate::line::Sticker;

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

pub async fn download_animated_stickers(pack_id: u32, stickers: &[Sticker], pack_name: &str) {
    for sticker in stickers {
        let apng_url = format!(
            "https://sdl-stickershop.line.naver.jp/products/0/0/1/{}/iphone/animation/{}@2x.png",
            pack_id, sticker.id
        );
        let apng_path = PathBuf::from(format!("{}/{}.apng", pack_name, sticker.id));
        let gif_path = PathBuf::from(format!("{}/{}.gif", pack_name, sticker.id));

        save_image(&apng_url, &apng_path).await;
        convert_apng_to_gif(&apng_path, &gif_path).expect("Failed to convert APNG to GIF");

        fs::remove_file(&apng_path).await.expect("Failed to delete APNG file");
    }
}

pub async fn save_image(url: &str, path: &PathBuf) {
    match reqwest::get(url).await {
        Ok(response) if response.status().is_success() => {
            let mut file = fs::File::create(path)
                .await
                .expect("Failed to create file");
            let content = response.bytes().await.expect("Failed to get image bytes");
            file.write_all(&content).await.expect("Failed to save image");
        }
        Ok(_) => eprintln!("Failed to download image from URL: {}", url),
        Err(err) => eprintln!("Failed to download image: {}", err),
    }
}

pub fn convert_apng_to_gif(apng_path: &PathBuf, gif_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let status = std::process::Command::new("ffmpeg")
        .args(&[
            "-i", apng_path.to_str().unwrap(),
            "-vf", "scale=iw:-1",
            "-vf", "palettegen=stats_mode=diff",
            "-y",
            "palette.png",
        ])
        .status()?;

    if !status.success() {
        return Err("ffmpeg failed to generate palette".into());
    }

    let status = std::process::Command::new("ffmpeg")
        .args(&[
            "-i", apng_path.to_str().unwrap(),
            "-i", "palette.png",
            "-lavfi", "paletteuse=dither=bayer:bayer_scale=3",
            "-y",
            gif_path.to_str().unwrap(),
        ])
        .status()?;

    std::fs::remove_file("palette.png").expect("Failed to remove temporary palette file");

    if !status.success() {
        return Err("ffmpeg failed to convert APNG to GIF".into());
    }

    Ok(())
}
