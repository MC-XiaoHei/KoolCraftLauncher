use std::sync::Mutex;
use tauri::State;
use tauri_plugin_os::Version::Semantic;
use crate::setup;

#[tauri::command]
pub fn get_vibrancy_state(state: State<'_, VibrancyStateStore>) -> String {
    state.get()
}

#[tauri::command]
pub fn should_custom_window() -> bool {
    let os_version = tauri_plugin_os::version();
    match os_version {
        Semantic(10, _, patch) => {
            !setup::is_win11(patch)
        }
        _ => true,
    }
}

pub struct VibrancyStateStore {
    pub state: Mutex<VibrancyState>,
}

#[allow(dead_code)]
pub enum VibrancyState {
    None,
    Blur,
    Acrylic,
    Mica,
    Vibrancy,
}

impl VibrancyStateStore {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(VibrancyState::None),
        }
    }

    pub fn set(&self, state: VibrancyState) {
        *self.state.lock().unwrap() = state;
    }

    pub fn get(&self) -> String {
        match *self.state.lock().unwrap() {
            VibrancyState::None => "none",
            VibrancyState::Blur => "blur",
            VibrancyState::Acrylic => "acrylic",
            VibrancyState::Mica => "mica",
            VibrancyState::Vibrancy => "vibrancy",
        }
            .to_string()
    }
}
