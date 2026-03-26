# Traktor-Pio-Link 🏆🚀

**Traktor-Pio-Link** is a professional-grade BPM bridge designed to synchronize Traktor Pro 4 (and 3) with High-End Pioneer DJ mixers (DJM-S7, S9, S11) using high-precision MIDI SysEx messages.

![App Icon](src-tauri/icons/128x128.png)

## 🌟 Key Features

- **Dual SysEx Synchronization**: Sends BPM updates using both ID 17 and ID 18 simultaneously for maximum hardware compatibility.
- **Auto-Scan & Fast Connect**: Automatically detects Pioneer MIDI ports and establishes a low-latency connection.
- **Integrated Mod Exporter**: A "one-click" solution to generate and export Traktor QML mods and MIDI mappings (.tsi).
- **Pro Interface**: Compact and aesthetic UI inspired by the CDJ-2000NXS2 layout.
- **Cross-Platform**: Native builds for both macOS (Silicon/Intel) and Windows.

---

## 🛠 Setup & Installation

### 1. Download the App
Go to the **[Releases](https://github.com/donmurdoc/traktor-pio-link/releases)** section and download the installer for your OS:
- **macOS**: `.dmg`
- **Windows**: `.exe` or `.msi`

### 2. Export & Install Mapping Files
1. Open **Traktor-Pio-Link**.
2. Click the **"MAPPING FILES"** button.
3. Select a destination (e.g., your Desktop).
4. You will get two items:
   - A folder named **`D2`**.
   - A file named **`s9-PAD-modes-modifier-template.tsi`**.

### 3. Configure Traktor
1. **MIDI Mapping**: In Traktor, go to `Settings > Controller Manager > Add... > Import TSI` and select the `.tsi` file exported in step 2.
2. **QML Mod**: Copy the generated **`D2`** folder to your Traktor installation directory:
   - **macOS**: `/Applications/Native Instruments/Traktor Pro 4/Traktor Pro 4.app/Contents/Resources/qml/CSI/D2`
   - **Windows**: `C:\Program Files\Native Instruments\Traktor Pro 4\Resources\qml\CSI\D2`
   *(Note: Backup your original D2 folder before replacing)*.
3. In Traktor, go to `Settings > Controller Manager > Add... > Traktor > ` and select D2

### 4. Connect to Mixer
- Ensure your Pioneer mixer is connected via USB.
- In the app, select the mixer from the **MIXER** dropdown.
- When Traktor is playing, the **MASTER BPM** display will light up and your mixer's effects will sync perfectly.

---

## 🧪 Technical Details

- **Backend**: Rust (Axum + Midir) for ultra-low latency.
- **Frontend**: Tauri + Vanilla CSS (Flat Design).
- **Protocol**: HTTP/JSON from Traktor QML to local server (Port 7001/7002).

---

## 🛡️ License & Privacy
This repository is **Private**. The source code is compiled into native binary for maximum intellectual property protection.

*Crafted for DJs by DJs.* 🎧🕶️✨
