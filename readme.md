# Screenshot Application

This application listens for keyboard events and captures screenshots when the left Ctrl key is pressed. Screenshots are saved in a designated directory with filenames based on the current date and time.

## Features

- **Automatic Screenshot Capture**: Press the left Ctrl key to take a screenshot of the entire screen.
- **Timestamped Filenames**: Screenshots are saved with filenames that include the current date and time, ensuring unique names for each capture.
- **Directory Management**: If the `screens` directory does not exist, the application will automatically create it.

## Prerequisites

- Rust programming language installed. You can install it from [rust-lang.org](https://www.rust-lang.org/).
- The following Rust crates are required:
  - `screenshots` for capturing screen images.
  - `rdev` for handling keyboard events.
  - `chrono` for date and time formatting.

## Installation

1. Clone the repository:
    ```bash
    git clone https://github.com/Saurus42/screens
    cd screens
    ```

2. Build the application:
    ```bash
    cargo build --release
    ```

## Usage

1. Run the application:
    ```bash
    cargo run --release
    ```

2. Follow the on-screen instructions:
   - To take a screenshot, press the left Ctrl key.
   - The list of created files will be displayed in real-time in the terminal.

3. Screenshots will be saved in the `screens` directory within the application folder.

## Code Overview

- **`main.rs`**: The main entry point of the application. It sets up event listening and manages key press events.
- **`callback` function**: Handles keyboard events, checks for left Ctrl key presses, captures the screen, and saves the screenshot with a timestamped filename.

## Error Handling

- If the `screens` directory cannot be created, an error message will be displayed in the terminal.
- Any errors during screen capture or file saving are also displayed in the terminal.

## Disclaimer

This application listens to all keyboard events, which may raise privacy concerns. Use this tool responsibly and ensure it aligns with your privacy and security requirements.
