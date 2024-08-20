use reqwest::Client;
use serde::Deserialize;
use std::fs;
use std::path::{PathBuf};
use std::process::Command;
use tokio;

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
struct Sticker {
    id: u32,
}

#[tokio::main]
async fn main() {
    println!("Enter the Sticker Pack ID:");
    let mut pack_id_input = String::new();
    std::io::stdin().read_line(&mut pack_id_input).expect("Failed to read line");
    let pack_id: u32 = pack_id_input.trim().parse().expect("Please enter a valid number");

    let pack_meta = fetch_pack_metadata(pack_id).await;
    let pack_name = sanitize_and_create_folder(&pack_meta.title.en);

    let contains_animated = check_for_animated_stickers(pack_id, &pack_meta.stickers).await;

    if contains_animated {
        // Ask the user for the desired file type if animations are present
        println!("Select the type of files you want to download: png, gif, or both");
        let mut file_type = String::new();
        std::io::stdin().read_line(&mut file_type).expect("Failed to read line");
        let file_type = file_type.trim().to_lowercase();

        download_stickers(pack_id, &pack_meta.stickers, &pack_name, &file_type).await;
    } else {
        // If only PNGs present, skip file type choice
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

fn sanitize_and_create_folder(name: &str) -> String {
    let sanitized_name: String = name
        .chars()
        .filter(|&c| !r#"/\:*?"<>|"#.contains(c))
        .collect();

    let path = PathBuf::from(&sanitized_name);
    if !path.exists() {
        fs::create_dir_all(&path).expect("Failed to create folder");
    }

    sanitized_name
}

async fn check_for_animated_stickers(pack_id: u32, stickers: &[Sticker]) -> bool {
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

async fn download_stickers(pack_id: u32, stickers: &[Sticker], pack_name: &str, pack_ext: &str) {
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

async fn download_static_stickers(stickers: &[Sticker], pack_name: &str) {
    for sticker in stickers {
        let url = format!(
            "http://dl.stickershop.line.naver.jp/stickershop/v1/sticker/{}/iphone/sticker@2x.png",
            sticker.id
        );
        let path = PathBuf::from(format!("{}/{}.png", pack_name, sticker.id));
        save_image(&url, &path).await;
    }
}

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
        fs::remove_file(&apng_path).expect("Failed to delete APNG file");
    }
}

async fn save_image(url: &str, path: &PathBuf) {
    match reqwest::get(url).await {
        Ok(response) if response.status().is_success() => {
            let mut file = tokio::fs::File::create(path)
                .await
                .expect("Failed to create file");
            let content = response.bytes().await.expect("Failed to get image bytes");
            tokio::io::copy(&mut content.as_ref(), &mut file)
                .await
                .expect("Failed to save image");
        }
        Ok(_) => eprintln!("Failed to download image from URL: {}", url),
        Err(err) => eprintln!("Failed to download image: {}", err),
    }
}

fn convert_apng_to_gif(apng_path: &PathBuf, gif_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("ffmpeg")
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

    let status = Command::new("ffmpeg")
        .args(&[
            "-i", apng_path.to_str().unwrap(),
            "-i", "palette.png",
            "-lavfi", "paletteuse=dither=bayer:bayer_scale=3",
            "-y",
            gif_path.to_str().unwrap(),
        ])
        .status()?;

    fs::remove_file("palette.png").expect("Failed to remove temporary palette file");

    if !status.success() {
        return Err("ffmpeg failed to convert APNG to GIF".into());
    }

    Ok(())
}
