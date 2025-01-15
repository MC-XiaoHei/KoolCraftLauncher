use std::sync::Mutex;

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
