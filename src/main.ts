import { invoke } from "@tauri-apps/api/core";

interface AudioFile {
  name: string;
  path: string;
}

const soundGrid = document.querySelector("#sound-grid") as HTMLElement;
const minimizeBtn = document.querySelector("#minimize");
const closeBtn = document.querySelector("#close");

async function setupWindowControls() {
  minimizeBtn?.addEventListener("click", () => {
    invoke("minimize_window");
  });

  closeBtn?.addEventListener("click", () => {
    invoke("close_window");
  });
}

function createSoundCard(audio: AudioFile) {
  const card = document.createElement("div");
  card.className = "sound-card";
  card.innerHTML = `
    <span class="sound-name">${audio.name}</span>
    <svg class="play-icon" viewBox="0 0 24 24">
      <path d="M8 5v14l11-7z"/>
    </svg>
  `;

  card.addEventListener("click", () => {
    invoke("play_audio", { path: audio.path });
  });

  return card;
}

async function loadSounds() {
  try {
    const files = await invoke<AudioFile[]>("get_audio_files");
    
    if (files.length === 0) {
      soundGrid.innerHTML = '<div style="text-align: center; color: #666; margin-top: 20px;">No sounds found in ./audios/</div>';
      return;
    }

    soundGrid.innerHTML = "";
    files.forEach((file) => {
      soundGrid.appendChild(createSoundCard(file));
    });
  } catch (error) {
    console.error("Failed to load sounds:", error);
  }
}

window.addEventListener("DOMContentLoaded", () => {
  setupWindowControls();
  loadSounds();
});
