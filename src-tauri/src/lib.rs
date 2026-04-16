use std::fs;
use std::path::PathBuf;
use serde::Serialize;
use tauri::{AppHandle, Manager, Runtime};

#[derive(Serialize)]
pub struct AudioFile {
    name: String,
    path: String,
}

#[tauri::command]
fn get_audio_files() -> Vec<AudioFile> {
    let mut files = Vec::new();
    let audio_dir = PathBuf::from("./audios");

    if !audio_dir.exists() {
        let _ = fs::create_dir_all(&audio_dir);
    }

    if let Ok(entries) = fs::read_dir(audio_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(extension) = path.extension() {
                if extension == "mp3" || extension == "wav" {
                    if let Some(name) = path.file_stem() {
                        files.push(AudioFile {
                            name: name.to_string_lossy().to_string(),
                            path: path.to_string_lossy().to_string(),
                        });
                    }
                }
            }
        }
    }
    files
}

#[tauri::command]
fn play_audio(path: String) {
    println!("Playing: {}", path);
    // Audio playback logic will go here
}

#[tauri::command]
async fn minimize_window<R: Runtime>(app: AppHandle<R>) -> Result<(), tauri::Error> {
  if let Some(window) = app.get_webview_window("main") {
    window.minimize()?;
  }
  Ok(())
}

#[tauri::command]
async fn close_window<R: Runtime>(app: AppHandle<R>) -> Result<(), tauri::Error> {
  if let Some(window) = app.get_webview_window("main") {
    window.close()?;
  }
  Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_audio_files,
            play_audio,
            minimize_window,
            close_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
