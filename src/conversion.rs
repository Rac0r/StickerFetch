use std::process::Command;
use std::path::PathBuf;

/// Converts an APNG file to a GIF using ffmpeg.
pub fn convert_apng_to_gif(apng_path: &PathBuf, gif_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
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

    std::fs::remove_file("palette.png").expect("Failed to remove temporary palette file");

    if !status.success() {
        return Err("ffmpeg failed to convert APNG to GIF".into());
    }

    Ok(())
}
