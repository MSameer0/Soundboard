import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { PhysicalPosition } from "@tauri-apps/api/dpi";
import { openUrl } from "@tauri-apps/plugin-opener";

interface AudioFile {
  name: string;
  path: string;
}

const soundGrid = document.querySelector("#sound-grid") as HTMLElement;
const minimizeBtn = document.querySelector("#minimize");
const closeBtn = document.querySelector("#close");
const stopBtn = document.querySelector("#stop-all");
const titlebar = document.querySelector("#titlebar") as HTMLElement;
const deviceSelect = document.querySelector("#output-device") as HTMLSelectElement;
const vbCableBanner = document.querySelector("#vb-cable-banner") as HTMLElement;
const downloadVbCableBtn = document.querySelector("#download-vb-cable");
const refreshDevicesBtn = document.querySelector("#refresh-devices");

// Custom window drag (bypasses Windows Aero Shake)
let isDragging = false;
let dragOffsetX = 0;
let dragOffsetY = 0;

titlebar?.addEventListener("mousedown", async (e) => {
  if ((e.target as HTMLElement).closest(".window-controls")) return;
  isDragging = true;
  const pos = await getCurrentWindow().outerPosition();
  dragOffsetX = e.screenX - pos.x;
  dragOffsetY = e.screenY - pos.y;
});

document.addEventListener("mousemove", (e) => {
  if (!isDragging) return;
  getCurrentWindow().setPosition(
    new PhysicalPosition(e.screenX - dragOffsetX, e.screenY - dragOffsetY)
  );
});

document.addEventListener("mouseup", () => {
  isDragging = false;
});

async function setupWindowControls() {
  minimizeBtn?.addEventListener("click", () => {
    invoke("minimize_window");
  });

  closeBtn?.addEventListener("click", () => {
    invoke("close_window");
  });

  stopBtn?.addEventListener("click", () => {
    invoke("stop_audio");
  });

  downloadVbCableBtn?.addEventListener("click", () => {
    openUrl("https://vb-audio.com/Cable/");
  });

  refreshDevicesBtn?.addEventListener("click", () => {
    // Optional: could add some animation feedback here
    loadDevices();
  });
}

async function loadDevices() {
  try {
    const devices = await invoke<string[]>("get_output_devices");
    deviceSelect.innerHTML = '<option value="">Default (Speakers)</option>';
    
    let vbCableFound = false;
    let vbCableName = "";

    devices.forEach((name) => {
      const option = document.createElement("option");
      option.value = name;
      option.textContent = name;
      deviceSelect.appendChild(option);

      const lowerName = name.toLowerCase();
      // Check for VB-Audio Virtual Cable string
      if (lowerName.includes("cable input") || lowerName.includes("vb-audio virtual cable")) {
        vbCableFound = true;
        vbCableName = name;
      }
    });

    if (vbCableFound) {
      // Hide banner, auto-select VB-Cable
      if (vbCableBanner) vbCableBanner.style.display = "none";
      deviceSelect.value = vbCableName;
      invoke("set_output_device", { name: vbCableName });
    } else {
      // Show setup banner, revert to default
      if (vbCableBanner) vbCableBanner.style.display = "flex";
      deviceSelect.value = "";
      invoke("set_output_device", { name: "" });
    }

  } catch (error) {
    console.error("Failed to load devices:", error);
  }
}

deviceSelect?.addEventListener("change", () => {
  const selected = deviceSelect.value;
  if (selected) {
    invoke("set_output_device", { name: selected });
  } else {
    // Reset to default — pass empty string which backend treats as None
    invoke("set_output_device", { name: "" });
  }
});

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
  loadDevices();
  loadSounds();
});
