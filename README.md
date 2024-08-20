# Line Downloader

Line Downloader is a command-line utility for downloading stickers from the LINE messaging app. This program allows users to download both static PNG stickers and animated GIFs from a specified sticker pack. It organizes downloaded stickers into a directory named after the sticker pack.

## Features

- Download static PNG stickers.
- Download animated stickers as GIFs.
- Save all stickers in a directory named after the sticker pack.
- Supports handling of transparent backgrounds in GIFs.

## Requirements

- **FFmpeg**: This tool is required for converting APNG files to GIFs. You can download FFmpeg from [ffmpeg.org](https://ffmpeg.org/download.html). Make sure `ffmpeg` is in your system's PATH.

## Download and Usage

1. **Download the Executable**:

   Download the latest release from the [GitHub Releases page](https://github.com/yourusername/line-downloader/releases). Choose the appropriate version for your operating system.

2. **Run the Program**:

   After downloading the executable, navigate to the directory where the executable is located.

   **On Windows**:

   Open Command Prompt and run:
   ```sh
   line-downloader.exe
   ```
   
    **On macOS/Linux**:
    
    Open the terminal and run:
    ```sh
    ./line-downloader
    ```

3. **Input Sticker Pack ID:**

    You will be prompted to enter the ID of the sticker pack you want to download. Enter the ID and press Enter.

4. **Select File Type**:

    After entering the sticker pack ID you can choose the file type:

    - PNG: Download only static PNG stickers.
    - GIF: Download only animated stickers as GIFs.
    - Both: Download both static PNG stickers and animated stickers as GIFs.

    If the sticker pack only contains static stickers the file type selection will be skipped
    and directly download the stickers as PNGs.

## Finding the Sticker Pack ID

