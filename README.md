# SoundBoard

This is a simple soundboard application built using Tauri and Rust. It's designed to be fast and lightweight, letting you play audio files instantly during your streams or games.

[![GitHub Release](https://img.shields.io/github/v/release/MSameer0/Soundboard?style=for-the-badge&color=4a90e2)](https://github.com/MSameer0/Soundboard/releases)
[![Platform](https://img.shields.io/badge/platform-windows-lightgrey?style=for-the-badge&logo=windows)](https://github.com/MSameer0/Soundboard/releases)
[![License](https://img.shields.io/badge/license-MIT-green?style=for-the-badge)](LICENSE)

---

## Features

- Instant audio playback using the Rodio engine.
- Virtual routing support for VB-Cable, so you can send audio through your mic.
- A "Stop All" button to silence everything immediately.
- Automatic detection of any audio files placed in the audios folder.
- Custom window dragging that won't trigger Windows Aero Shake.

---

## Technical Stack

This project uses the following technologies:

![Tauri](https://img.shields.io/badge/Tauri-FFC131?style=for-the-badge&logo=tauri&logoColor=white)
![Rust](https://img.shields.io/badge/rust-%23E05D44.svg?style=for-the-badge&logo=rust&logoColor=white)
![TypeScript](https://img.shields.io/badge/typescript-%23007ACC.svg?style=for-the-badge&logo=typescript&logoColor=white)
![Vite](https://img.shields.io/badge/vite-%23646CFF.svg?style=for-the-badge&logo=vite&logoColor=white)
![CSS3](https://img.shields.io/badge/css3-%231572B6.svg?style=for-the-badge&logo=css3&logoColor=white)

---

## Getting Started

### Installation

1.  Go to the Releases page on GitHub.
2.  Download the latest installer (.msi or .exe).
3.  Run the setup and you're good to go.

### Audio Setup (VB-Cable)

If you want others to hear your sounds in Discord or other apps, you'll need a virtual cable.

1.  Download and install VB-Cable from the official site.
2.  Restart your computer so the new audio device shows up.
3.  Inside the SoundBoard app, set the Output Device to "CABLE Input".
4.  In your recording app (like Discord), set your microphone input to "CABLE Output".

---

## How to Use

1.  Put your audio files (.mp3, .wav, or .ogg) into the "audios" folder next to the app.
2.  Launch the application and your sounds will show up in the grid.
3.  Click a card to play it.
4.  Use the red "Stop All" button at the top to stop any sounds currently playing.

---

## Contact

- GitHub: [MSameer0](https://github.com/MSameer0)
- LinkedIn: [Muhammad Sameer Adnan](https://www.linkedin.com/in/muhammadsameeradnan)

---

## License

This project is licensed under the MIT License. Check the LICENSE file for more details.
