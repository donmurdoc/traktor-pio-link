# Traktor-Pio-Link 

**Traktor-Pio-Link** is a BPM bridge designed to synchronize Traktor Pro 4 (and 3) with Pioneer DJ mixers (DJM-S7, S9, S11) using MIDI SysEx messages.

![App Icon](src-tauri/icons/128x128.png)

[![Ko-fi](https://img.shields.io/badge/Ko--fi-F16061?style=for-the-badge&logo=ko-fi&logoColor=white)](https://ko-fi.com/traktorpiolink) 
*I could really use a tip, I'm so hungry 🍕😿*

---

## 🛠 Setup Guide

### 1. Download the App
Go to the **[Releases](https://github.com/lautrip/traktor-pio-link/releases)** section and download the installer for your OS:
- **macOS**: `.dmg`
- **Windows**: `.exe` or `.msi`

### 2. Export & Install Mapping Files
1. Open **Traktor-Pio-Link** and click the **"MAPPING FILES"** button.
2. Select a destination (e.g. your Desktop). You will get:
   - **D2 folder**
   - **s9-PAD-modes-modifier-template.tsi**

### 3. Configure Traktor
- **MIDI Mapping — In Traktor:**
  `Settings > Controller Manager > Add... > Import TSI` -> select the `.tsi` file from Step 2. Then select for midi in and out your mixer (S9,S7,S11). You only need this to activate pad modes.
- **QML Mod — Copy the D2 folder to your Traktor directory:**
  - **macOS**: `/Applications/Native Instruments/Traktor Pro 4/Traktor Pro 4.app/Contents/Resources/qml/CSI/D2`
  - **Windows**: `C:\Program Files\Native Instruments\Traktor Pro 4\Resources\qml\CSI\D2`
  *(Note: Backup your original D2 folder before replacing it).*
- **Then, in Traktor:**
  `Settings > Controller Manager > Add... > Pre Mapped > Traktor Kontrol > D2`

### 4. Connect to Mixer
- Ensure your Pioneer mixer is connected via USB.
- In the app, select the mixer from the **MIXER** dropdown.

When Traktor is playing, the **MASTER BPM** display will light up and your mixer's effects will sync perfectly. **You're all set!**

---

## 🧪 Technical Details

- **Backend**: Rust (Axum + Midir) for ultra-low latency.
- **Frontend**: Tauri + Vanilla CSS (Flat Design).
- **Protocol**: HTTP/JSON from Traktor QML to local server (Port 7001/7002).

---

## ☕ Support & Donations

If this project helps your DJ sets, consider buying me a coffee!
**[ko-fi.com/traktorpiolink](https://ko-fi.com/traktorpiolink)**

> I'm also very good at making MIDI mappings, and I have killer mappings for the S9. DM if you want it!

## ⚠️ Disclaimer

This is an experimental tool that interacts with professional hardware via MIDI SysEx protocols. Use it at your own risk.

- **No Warranty**: The software is provided "as is". The author makes no guarantees regarding its stability or compatibility with future hardware/software versions.
- **Limitation of Liability**: The author shall not be held responsible for any damage to your computer, DJ equipment (CDJs, mixers, etc.), data loss, or any technical issues occurring during live performances.
- **Testing**: It is strongly recommended to test this tool thoroughly in a safe, non-live environment before using it in a professional setting.

---

## 🛡️ License
This project is licensed under the [MIT License](LICENSE). 

*Crafted for DJs by DJs.* 🎧🕶️✨
