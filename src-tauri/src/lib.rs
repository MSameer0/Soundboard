use std::fs::{self, File};
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use serde::Serialize;
use tauri::{AppHandle, Manager, Runtime};
use rodio::{Decoder, OutputStream, Sink};
use cpal::traits::{DeviceTrait, HostTrait};

#[derive(Serialize)]
pub struct AudioFile {
    name: String,
    path: String,
}

struct AudioState {
    sinks: Arc<Mutex<Vec<Arc<Sink>>>>,
    selected_device: Arc<Mutex<Option<String>>>,
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
                if extension == "mp3" || extension == "wav" || extension == "flac" {
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
fn get_output_devices() -> Vec<String> {
    let host = cpal::default_host();
    let mut devices = Vec::new();
    if let Ok(output_devices) = host.output_devices() {
        for device in output_devices {
            if let Ok(name) = device.name() {
                devices.push(name);
            }
        }
    }
    devices
}

#[tauri::command]
fn set_output_device(name: String, state: tauri::State<'_, AudioState>) {
    if let Ok(mut selected) = state.selected_device.lock() {
        if name.is_empty() {
            println!("Output device reset to default");
            *selected = None;
        } else {
            println!("Output device set to: {}", name);
            *selected = Some(name);
        }
    }
}

#[tauri::command]
fn play_audio(path: String, app: AppHandle) {
    println!("Playing: {}", path);

    let sinks_handle = app.state::<AudioState>().sinks.clone();
    let device_name = app.state::<AudioState>().selected_device.lock()
        .ok()
        .and_then(|d| d.clone());

    std::thread::spawn(move || {
        let file = match File::open(&path) {
            Ok(f) => f,
            Err(_) => {
                println!("Failed to open file");
                return;
            }
        };

        // Open output stream on selected device (or default)
        let (_stream, stream_handle) = if let Some(ref dev_name) = device_name {
            let host = cpal::default_host();
            let device = host.output_devices()
                .ok()
                .and_then(|mut devices| devices.find(|d| {
                    d.name().map(|n| n == *dev_name).unwrap_or(false)
                }));
            match device {
                Some(dev) => {
                    match OutputStream::try_from_device(&dev) {
                        Ok(stream) => stream,
                        Err(e) => {
                            println!("Failed to open device '{}': {:?}, falling back to default", dev_name, e);
                            OutputStream::try_default().unwrap()
                        }
                    }
                }
                None => {
                    println!("Device '{}' not found, using default", dev_name);
                    OutputStream::try_default().unwrap()
                }
            }
        } else {
            OutputStream::try_default().unwrap()
        };

        let sink = Arc::new(Sink::try_new(&stream_handle).unwrap());

        let source = match Decoder::new(BufReader::new(file)) {
            Ok(s) => s,
            Err(e) => {
                println!("Decode failed: {:?}", e);
                return;
            }
        };

        sink.append(source);

        if let Ok(mut sinks) = sinks_handle.lock() {
            sinks.push(Arc::clone(&sink));
        }

        sink.sleep_until_end();

        if let Ok(mut sinks) = sinks_handle.lock() {
            sinks.retain(|s| !Arc::ptr_eq(s, &sink));
        }
    });
}

#[tauri::command]
fn stop_audio(state: tauri::State<'_, AudioState>) {
    println!("Stopping all audio");
    if let Ok(mut sinks) = state.sinks.lock() {
        for sink in sinks.iter() {
            sink.stop();
        }
        sinks.clear();
    }
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
        .manage(AudioState {
            sinks: Arc::new(Mutex::new(Vec::new())),
            selected_device: Arc::new(Mutex::new(None)),
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_audio_files,
            get_output_devices,
            set_output_device,
            play_audio,
            stop_audio,
            minimize_window,
            close_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
