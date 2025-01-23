use crate::vibrancy::VibrancyState;
use tauri::App;

#[cfg(target_os = "windows")]
use tauri_plugin_os::Version::Semantic;
#[cfg(any(target_os = "macos", target_os = "windows"))]
use window_vibrancy::*;
#[cfg(any(target_os = "macos", target_os = "windows"))]
use tauri::Manager;

pub fn can_use_acrylic(patch: u64) -> bool {
    const EARLIEST_WIN_10_PATCH_VERSION_CAN_ACRYLIC: u64 = 18090;
    const LATEST_WIN_10_PATCH_VERSION_CAN_ACRYLIC: u64 = 19039;
    patch <= LATEST_WIN_10_PATCH_VERSION_CAN_ACRYLIC
        && patch >= EARLIEST_WIN_10_PATCH_VERSION_CAN_ACRYLIC
}

pub fn is_win11(patch: u64) -> bool {
    const EARLIEST_WIN_11_PATCH_VERSION: u64 = 22000;
    patch >= EARLIEST_WIN_11_PATCH_VERSION
}

#[allow(unused_variables)]
pub fn setup_window(app: &mut App) -> Result<VibrancyState, Box<dyn std::error::Error>> {
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    let win = app.get_webview_window("main").unwrap();

    #[cfg(target_os = "macos")]
    match apply_vibrancy(&win, NSVisualEffectMaterial::HudWindow, None, None) {
        Ok(_) => return Ok(VibrancyState::Vibrancy),
        Err(_) => {}
    }

    #[cfg(target_os = "windows")]
    {
        let os_version = tauri_plugin_os::version();
        match os_version {
            Semantic(10, _, patch) if is_win11(patch) => {
                match apply_mica(&win, Some(true)) {
                    Ok(_) => return Ok(VibrancyState::Mica),
                    Err(_) => {}
                }
            }
            Semantic(10, _, patch) => {
                win.set_shadow(false).unwrap();
                if can_use_acrylic(patch) {
                    match apply_acrylic(&win, Some((18, 18, 18, 125))) {
                        Ok(_) => return Ok(VibrancyState::Acrylic),
                        Err(_) => {}
                    }
                }
            }
            _ => {}
        }
    }

    Ok(VibrancyState::None)
}
