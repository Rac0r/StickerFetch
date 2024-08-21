use std::fs;
use std::path::{PathBuf};

pub fn sanitize_and_create_folder(name: &str) -> String {
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

pub async fn save_image(url: &str, path: &PathBuf) {
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
