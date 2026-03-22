const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

let bpmValueEl;
let midiSelectEl;
let statusDotEl;
let statusTextEl;
let portInputEl;

let lastPortsData = ""; 

async function updateMidiPorts() {
    try {
        const ports = await invoke("get_midi_ports");
        const filtered = ports
            .map((name, index) => ({ name, index }))
            .filter(item => {
                const n = item.name.toUpperCase();
                return (n.includes("S9") || n.includes("S11") || n.includes("S7")) && !n.includes("IAC");
            });

        const portsHash = JSON.stringify(filtered);
        if (portsHash === lastPortsData) return;
        
        lastPortsData = portsHash;
        const savedMixer = localStorage.getItem("selectedMixerName");

        midiSelectEl.innerHTML = '<option value="">Select device...</option>';
        filtered.forEach(item => {
            const option = document.createElement("option");
            option.value = item.index; 
            option.textContent = item.name;
            midiSelectEl.appendChild(option);
        });
        
        for (let item of filtered) {
            if (item.name === savedMixer) {
                if (midiSelectEl.value !== String(item.index)) {
                    midiSelectEl.value = item.index;
                    handleMidiSelection();
                }
                break;
            }
        }
    } catch (err) {
        // console.error("Error fetching MIDI ports:", err);
    }
}

async function handleMidiSelection() {
    const index = midiSelectEl.value;
    if (index === "") {
        localStorage.removeItem("selectedMixerName");
        statusDotEl.classList.remove("active");
        statusTextEl.textContent = "DISCONNECTED";
        return;
    }
    
    const selectedName = midiSelectEl.options[midiSelectEl.selectedIndex].text;
    localStorage.setItem("selectedMixerName", selectedName);

    try {
        await invoke("select_midi_port", { index: parseInt(index) });
        statusDotEl.classList.add("active");
        statusTextEl.textContent = "CONNECTED";
    } catch (err) {
        console.error("Error selecting MIDI port:", err);
        statusDotEl.classList.remove("active");
        statusTextEl.textContent = "MIDI ERROR";
    }
}

async function handlePortChange() {
    const port = parseInt(portInputEl.value);
    if (isNaN(port)) return;
    localStorage.setItem("selectedPort", port);
    try {
        await invoke("update_listen_port", { port });
    } catch (err) {
        console.error("Error updating port:", err);
    }
}

window.addEventListener("DOMContentLoaded", async () => {
    bpmValueEl = document.querySelector("#bpm_value");
    midiSelectEl = document.querySelector("#midi_select");
    statusDotEl = document.querySelector("#status_dot");
    statusTextEl = document.querySelector("#status_text");
    portInputEl = document.querySelector("#port_input");

    const savedPort = localStorage.getItem("selectedPort");
    if (savedPort) {
        portInputEl.value = savedPort;
        handlePortChange();
    }

    await updateMidiPorts();
    setInterval(updateMidiPorts, 2000);

    midiSelectEl.addEventListener("change", handleMidiSelection);
    portInputEl.addEventListener("change", handlePortChange);

    document.querySelector("#export_button").addEventListener("click", async () => {
        console.log("Button clicked!");
        try {
            const result = await invoke("export_traktor_mod");
            alert(result);
        } catch (err) {
            console.error("Export error:", err);
            alert("Export error: " + err);
        }
    });

    await listen("bpm-update", (event) => {
        const bpm = event.payload;
        bpmValueEl.textContent = typeof bpm === 'number' ? bpm.toFixed(1) : bpm;
    });
});
