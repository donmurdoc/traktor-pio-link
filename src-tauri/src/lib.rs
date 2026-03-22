use axum::{routing::post, Router};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::{Emitter, Manager};
use tower_http::cors::CorsLayer;
use midir::{MidiOutput, MidiOutputConnection};
use std::fs;
// --- Traktor Mod Source Constants ---
const D2_QML: &str = r#"import CSI 1.0
import QtQuick 2.0

import "../../Defines"
import "../Common"
import "../Common/Settings"
import "./Api"

Mapping
{
  ApiModule {}

  //------------------------------------------------------------------------------------------------------------------
  // LOOP/BEATJUMP SIZE SETTINGS
  //------------------------------------------------------------------------------------------------------------------
  LoopModePads
  {
    name: "loop_mode_pads"
    loopSizePath: "mapping.settings.pad_loop_size" 
    beatJumpPath: "mapping.settings.pad_jump_size" 
  }

  //------------------------------------------------------------------------------------------------------------------
  //
  //------------------------------------------------------------------------------------------------------------------

  // Settings - MIDI Controls
  MappingPropertyDescriptor { id: useMIDIControls;                  path: "mapping.settings.use_midi_controls";                  type: MappingPropertyDescriptor.Boolean;    value: false }

  // Settings - Touch Controls
  MappingPropertyDescriptor { id: showBrowserOnTouch;               path: "mapping.settings.show_browser_on_touch";              type: MappingPropertyDescriptor.Boolean;    value: false }
  MappingPropertyDescriptor { id: showFxOnTouch;                    path: "mapping.settings.show_fx_panels_on_touch";            type: MappingPropertyDescriptor.Boolean;    value: true  }
  MappingPropertyDescriptor { id: showPerformanceControlsOnTouch;   path: "mapping.settings.show_performance_control_on_touch";  type: MappingPropertyDescriptor.Boolean;    value: true  }

  // Settings - Touchstrip
  MappingPropertyDescriptor { id: scratchWithTouchstrip;            path: "mapping.settings.scratch_with_touchstrip";            type: MappingPropertyDescriptor.Boolean;    value: false }
  MappingPropertyDescriptor { path: "mapping.settings.touchstrip_bend_sensitivity";        type: MappingPropertyDescriptor.Float;      value: 50.0;   min: 0.0; max: 100.0; }
  MappingPropertyDescriptor { path: "mapping.settings.touchstrip_bend_invert";             type: MappingPropertyDescriptor.Boolean;    value: false;  }
  MappingPropertyDescriptor { path: "mapping.settings.touchstrip_scratch_sensitivity";     type: MappingPropertyDescriptor.Float;      value: 50.0;   min: 0.0; max: 100.0; }
  MappingPropertyDescriptor { path: "mapping.settings.touchstrip_scratch_invert";          type: MappingPropertyDescriptor.Boolean;    value: true;  }

  //------------------------------------------------------------------------------------------------------------------
  // CROSS-DISPLAY INTERACTION
  // Browser should be open on one display only
  //------------------------------------------------------------------------------------------------------------------

  /*
  property bool leftScreenViewValue: left.screenView.value
  property bool rightScreenViewValue: right.screenView.value

  onLeftScreenViewValueChanged:
  {
    if (left.screenView.value == ScreenView.browser && right.screenView.value == ScreenView.browser)
    {
      right.screenView.value = ScreenView.deck;
    }
  }

  onRightScreenViewValueChanged:
  {
    if (left.screenView.value == ScreenView.browser && right.screenView.value == ScreenView.browser)
    {
      left.screenView.value = ScreenView.deck;
    }
  }
  */

  //------------------------------------------------------------------------------------------------------------------

  onMappingLoaded:
  {
    deck.initializeModule();
  }

  //------------------------------------------------------------------------------------------------------------------
  //
  //------------------------------------------------------------------------------------------------------------------

  D2 { name: "surface" }

  //------------------------------------------------------------------------------------------------------------------
  //  LED Brightness wiring
  //------------------------------------------------------------------------------------------------------------------

  MappingPropertyDescriptor { path: "mapping.settings.led_on_brightness";      type: MappingPropertyDescriptor.Integer;   value: 100; min: 50; max: 100 }
  MappingPropertyDescriptor { path: "mapping.settings.led_dimmed_percentage";  type: MappingPropertyDescriptor.Integer;   value: 25;  min: 25; max: 50  }

  DirectPropertyAdapter { name: "LEDBrightnessOn";      path: "mapping.settings.led_on_brightness";      input: false }
  DirectPropertyAdapter { name: "LEDDimmedPercentage";  path: "mapping.settings.led_dimmed_percentage";  input: false }

  Wire { from: "surface.led_on_brightness.write";      to: "LEDBrightnessOn.read"     }
  Wire { from: "surface.led_dimmed_brightness.write";  to: "LEDDimmedPercentage.read" }

  //------------------------------------------------------------------------------------------------------------------

  Deck_S8Style
  {
    id: deck
    name: "deck"
    surface: "surface"
    settingsPath: "mapping.settings"
    propertiesPath: "mapping.state"
    useMIDIControls: useMIDIControls.value
    decksAssignment: decksAssignment.value
  }
 
  //------------------------------------------------------------------------------------------------------------------
  // Decks Assignment / Deck focus
  //------------------------------------------------------------------------------------------------------------------

  property bool isInDecksAssignmentMode: false
  property bool triggerDeckFocusSwitch:  true

  MappingPropertyDescriptor { id: decksAssignment; path: "mapping.settings.decks_assignment"; type: MappingPropertyDescriptor.Integer; value: DecksAssignment.AC; }

  ButtonScriptAdapter 
  { 
    name: "deck_button_adapter"
    onPress: 
    { 
      isInDecksAssignmentMode = true;
      triggerDeckFocusSwitch  = true
    } 
    onRelease:
    {
      if (triggerDeckFocusSwitch) deck.deckFocus = !deck.deckFocus;
      isInDecksAssignmentMode = false;
    }
    brightness: 1.0; 
    color: (deck.deckFocus ? Color.White : Color.Blue) 
  } 

  Wire { from: "surface.deck"; to: "deck_button_adapter" }

  WiresGroup
  {
    enabled: isInDecksAssignmentMode

    Wire 
    {
      from: "surface.fx.assign.1"
      to: ButtonScriptAdapter
      {
        onPress:
        {
          decksAssignment.value  = DecksAssignment.AC;
          deck.deckFocus         = false;
          triggerDeckFocusSwitch = false;
        }
        brightness: (decksAssignment.value == DecksAssignment.AC)
      }
    }

    Wire 
    {
      from: "surface.fx.assign.2"
      to: ButtonScriptAdapter
      {
        onPress:
        {
          decksAssignment.value  = DecksAssignment.BD;
          deck.deckFocus         = false;
          triggerDeckFocusSwitch = false;
        }
        brightness: (decksAssignment.value == DecksAssignment.BD)
      }
    }

    Wire 
    {
      from: "surface.fx.assign.3"
      to: ButtonScriptAdapter
      {
        onPress:
        {
          decksAssignment.value  = DecksAssignment.AC;
          deck.deckFocus         = true;
          triggerDeckFocusSwitch = false;
        }
        brightness: (decksAssignment.value == DecksAssignment.AC)
      }
    }

    Wire 
    {
      from: "surface.fx.assign.4"
      to: ButtonScriptAdapter
      {
        onPress:
        {
          decksAssignment.value  = DecksAssignment.BD;
          deck.deckFocus         = true;
          triggerDeckFocusSwitch = false;
        }
        brightness: (decksAssignment.value == DecksAssignment.BD)
      }
    }
  }

  //------------------------------------------------------------------------------------------------------------------
  // Fx Assignment
  //------------------------------------------------------------------------------------------------------------------

  AppProperty { id: fxMode; path: "app.traktor.fx.4fx_units" }

  WiresGroup
  {
    enabled: !isInDecksAssignmentMode

    WiresGroup
    {
      enabled: !deck.shift
      WiresGroup
      {
        enabled: decksAssignment.value == DecksAssignment.AC
        Wire { from: "surface.fx.assign.1"; to: TogglePropertyAdapter { path: "app.traktor.mixer.channels.1.fx.assign.1"; } }
        Wire { from: "surface.fx.assign.2"; to: TogglePropertyAdapter { path: "app.traktor.mixer.channels.2.fx.assign.1"; } }
        Wire { from: "surface.fx.assign.3"; to: TogglePropertyAdapter { path: "app.traktor.mixer.channels.3.fx.assign.1"; } }
        Wire { from: "surface.fx.assign.4"; to: TogglePropertyAdapter { path: "app.traktor.mixer.channels.4.fx.assign.1"; } }
      }

      WiresGroup
      {
        enabled: decksAssignment.value == DecksAssignment.BD
        Wire { from: "surface.fx.assign.1"; to: TogglePropertyAdapter { path: "app.traktor.mixer.channels.1.fx.assign.2"; } }
        Wire { from: "surface.fx.assign.2"; to: TogglePropertyAdapter { path: "app.traktor.mixer.channels.2.fx.assign.2"; } }
        Wire { from: "surface.fx.assign.3"; to: TogglePropertyAdapter { path: "app.traktor.mixer.channels.3.fx.assign.2"; } }
        Wire { from: "surface.fx.assign.4"; to: TogglePropertyAdapter { path: "app.traktor.mixer.channels.4.fx.assign.2"; } }
      }
    }

    WiresGroup
    {
      enabled: deck.shift && (fxMode.value == FxMode.FourFxUnits)
      WiresGroup
      {
        enabled: decksAssignment.value == DecksAssignment.AC
        Wire { from: "surface.fx.assign.1"; to: TogglePropertyAdapter { path: "app.traktor.mixer.channels.1.fx.assign.3"; } }
        Wire { from: "surface.fx.assign.2"; to: TogglePropertyAdapter { path: "app.traktor.mixer.channels.2.fx.assign.3"; } }
        Wire { from: "surface.fx.assign.3"; to: TogglePropertyAdapter { path: "app.traktor.mixer.channels.3.fx.assign.3"; } }
        Wire { from: "surface.fx.assign.4"; to: TogglePropertyAdapter { path: "app.traktor.mixer.channels.4.fx.assign.3"; } }
      }

      WiresGroup
      {
        enabled: decksAssignment.value == DecksAssignment.BD
        Wire { from: "surface.fx.assign.1"; to: TogglePropertyAdapter { path: "app.traktor.mixer.channels.1.fx.assign.4"; } }
        Wire { from: "surface.fx.assign.2"; to: TogglePropertyAdapter { path: "app.traktor.mixer.channels.2.fx.assign.4"; } }
        Wire { from: "surface.fx.assign.3"; to: TogglePropertyAdapter { path: "app.traktor.mixer.channels.3.fx.assign.4"; } }
        Wire { from: "surface.fx.assign.4"; to: TogglePropertyAdapter { path: "app.traktor.mixer.channels.4.fx.assign.4"; } }
      }
    }
  }
} //Mapping
"#;

const API_CLIENT_JS: &str = r#"
// ApiClient.js
// Multicast to both Visual Buddy and S9-S11 hack
var API_URLS = [
  "http://127.0.0.1:7002"  // Traktor-Pio Link
];

function send(endpoint, data) {
  var payload = JSON.stringify(data);
  for (var i = 0; i < API_URLS.length; i++) {
    var xhr = new XMLHttpRequest();
    xhr.open("POST", API_URLS[i] + "/" + endpoint, true);
    xhr.setRequestHeader("Content-Type", "application/json");
    xhr.send(payload);
  }
}
"#;

const API_MASTER_CLOCK_QML: &str = r#"
import CSI 1.0
import QtQuick 2.0
import "ApiClient.js" as ApiClient

Item {
  AppProperty { id: propMasterDeckId;  path: "app.traktor.masterclock.source_id";  onValueChanged: updateMasterClock() }
  AppProperty { id: propMasterBpm;     path: "app.traktor.masterclock.tempo";      onValueChanged: masterBpmChangedTimer.restart() }

  Timer {
    id: masterBpmChangedTimer
    interval: 250
    onTriggered: updateMasterClock()
  }

  function updateMasterClock() {
    ApiClient.send("updateMasterClock", {
      deck: (propMasterDeckId.value == -1) ? null : String.fromCharCode(65 + propMasterDeckId.value),
      bpm: propMasterBpm.value,
    })
  }
}
"#;

const API_MODULE_QML: &str = r#"
// ApiModule.qml
import CSI 1.0
Module {
  ApiMasterClock {} 
}
"#;

#[derive(Clone)]
pub struct AppState {
    pub current_bpm: Arc<Mutex<f32>>,
    pub midi_connection: Arc<Mutex<Option<MidiOutputConnection>>>,
    pub app_handle: tauri::AppHandle,
    pub server_handle: Arc<Mutex<Option<tauri::async_runtime:: JoinHandle<()>>>>,
    pub current_port: Arc<Mutex<u16>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateDeckPayload {
    #[serde(default, alias = "currentBpm", alias = "current_bpm", alias = "bpm")]
    pub current_bpm: f32,
    #[serde(default, alias = "is_playing", alias = "isPlaying")]
    pub is_playing: bool,
}

#[derive(Clone)]
struct AxumState {
    app: tauri::AppHandle,
}

fn calculate_sysex(bpm: f32, device_id: u8) -> Vec<u8> {
    let total_value = (bpm * 100.0).round() as u32;
    let mut msg = vec![240, 0, 32, 127, device_id, 0, 0];
    msg.push(((total_value >> 12) & 0x0F) as u8);
    msg.push(((total_value >> 8) & 0x0F) as u8);
    msg.push(((total_value >> 4) & 0x0F) as u8);
    msg.push((total_value & 0x0F) as u8);
    msg.push(247);
    msg
}

async fn on_update_deck(
    axum::extract::State(s): axum::extract::State<AxumState>,
    axum::extract::Json(payload): axum::extract::Json<UpdateDeckPayload>,
) -> &'static str {
    let app = s.app;
    let bpm = payload.current_bpm;
    
    if let Some(state) = app.try_state::<AppState>() {
        *state.current_bpm.lock().unwrap() = bpm;
        let _ = app.emit("bpm-update", bpm);
        
        let mut conn_guard = state.midi_connection.lock().unwrap();
        if let Some(conn) = conn_guard.as_mut() {
            let sysex_17 = calculate_sysex(bpm, 17);
            let _ = conn.send(&sysex_17);
            
            let sysex_18 = calculate_sysex(bpm, 18);
            let _ = conn.send(&sysex_18);

            let hex_msg = hex::encode(&sysex_17);
            let _ = app.emit("sysex-sent", hex_msg);
        }
    }
    "OK"
}

#[tauri::command]
fn get_midi_ports() -> Result<Vec<String>, String> {
    let midi_out = MidiOutput::new("BPM SysEx Bridge").map_err(|e| e.to_string())?;
    let ports = midi_out.ports();
    let mut names = Vec::new();
    for port in ports {
        if let Ok(name) = midi_out.port_name(&port) {
            names.push(name);
        }
    }
    Ok(names)
}

#[tauri::command]
fn select_midi_port(index: usize, state: tauri::State<AppState>) -> Result<(), String> {
    let midi_out = MidiOutput::new("BPM SysEx Bridge").map_err(|e| e.to_string())?;
    let ports = midi_out.ports();
    if index >= ports.len() {
        return Err("Invalid port index".into());
    }
    let port = &ports[index];
    let conn = midi_out.connect(port, "bpm-sysex-out").map_err(|e| e.to_string())?;
    *state.midi_connection.lock().unwrap() = Some(conn);
    Ok(())
}

#[tauri::command]
fn export_traktor_mod(_app: tauri::AppHandle) -> Result<String, String> {
    println!("🚀 [Rust] export_traktor_mod called (with Dialog)");
    
    let folder = rfd::FileDialog::new()
        .set_title("Select Destination for Traktor Map Files")
        .pick_folder();

    let base_path = match folder {
        Some(p) => p,
        None => return Err("Export cancelled by user".into()),
    };
    
    let target_dir = base_path.join("D2");
    println!("📂 [Rust] Target directory: {:?}", target_dir);

    fs::create_dir_all(target_dir.join("Api")).map_err(|e| {
        let err_msg = format!("Could not create directory: {}", e);
        println!("❌ [Rust] {}", err_msg);
        err_msg
    })?;

    fs::write(target_dir.join("D2.qml"), D2_QML).map_err(|e| {
        let err_msg = format!("Error writing D2.qml: {}", e);
        println!("❌ [Rust] {}", err_msg);
        err_msg
    })?;
    fs::write(target_dir.join("Api/ApiClient.js"), API_CLIENT_JS).map_err(|e| {
        let err_msg = format!("Error writing ApiClient.js: {}", e);
        println!("❌ [Rust] {}", err_msg);
        err_msg
    })?;
    fs::write(target_dir.join("Api/ApiMasterClock.qml"), API_MASTER_CLOCK_QML).map_err(|e| {
        let err_msg = format!("Error writing ApiMasterClock.qml: {}", e);
        println!("❌ [Rust] {}", err_msg);
        err_msg
    })?;
    fs::write(target_dir.join("Api/ApiModule.qml"), API_MODULE_QML).map_err(|e| {
        let err_msg = format!("Error writing ApiModule.qml: {}", e);
        println!("❌ [Rust] {}", err_msg);
        err_msg
    })?;

    let success_msg = format!("✅ Success! Exported to: {:?}", target_dir);
    println!("{}", success_msg);
    Ok(success_msg)
}

#[tauri::command]
async fn update_listen_port(port: u16, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut port_guard = state.current_port.lock().unwrap();
    if *port_guard == port {
        return Ok(());
    }
    *port_guard = port;
    
    let mut server_guard = state.server_handle.lock().unwrap();
    if let Some(handle) = server_guard.take() {
        handle.abort();
    }
    
    let app_handle = state.app_handle.clone();
    let new_handle = tauri::async_runtime::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        let _ = start_server(app_handle, port).await;
    });
    *server_guard = Some(new_handle);
    Ok(())
}

async fn start_server(app_handle: tauri::AppHandle, port: u16) -> Result<(), String> {
    let axum_state = AxumState { app: app_handle };
    let app_router = Router::new()
        .route("/", post(on_update_deck))
        .route("/updateMasterClock", post(on_update_deck))
        .layer(CorsLayer::permissive())
        .with_state(axum_state);

    let addr = format!("127.0.0.1:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await
        .map_err(|e| format!("Could not bind to port {}: {}", port, e))?;
    
    axum::serve(listener, app_router).await
        .map_err(|e| format!("Server error: {}", e))
}

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let bpm = Arc::new(Mutex::new(0.0));
            let midi_connection = Arc::new(Mutex::new(None));
            let app_handle = app.handle().clone();
            
            let state = AppState {
                current_bpm: bpm.clone(),
                midi_connection,
                app_handle: app_handle.clone(),
                server_handle: Arc::new(Mutex::new(None)),
                current_port: Arc::new(Mutex::new(7001)),
            };
            app.manage(state.clone());

            let app_handle_for_server = app_handle.clone();
            let server_thread = tauri::async_runtime::spawn(async move {
                let _ = start_server(app_handle_for_server, 7001).await;
            });
            *state.server_handle.lock().unwrap() = Some(server_thread);

            // Periodically send a SysEx to keep the connection alive if needed
            let midi_conn_clone = state.midi_connection.clone();
            std::thread::spawn(move || {
                loop {
                    std::thread::sleep(std::time::Duration::from_millis(500));
                    let mut conn_guard = midi_conn_clone.lock().unwrap();
                    if let Some(conn) = conn_guard.as_mut() {
                        let _ = conn.send(&[0xF0, 0x00, 0x20, 0x7F, 0x50, 0x01, 0xF7]);
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_midi_ports,
            select_midi_port,
            update_listen_port,
            export_traktor_mod
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
