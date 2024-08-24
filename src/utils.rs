// src/utils.rs

use std::fs;
use std::path::PathBuf;

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
