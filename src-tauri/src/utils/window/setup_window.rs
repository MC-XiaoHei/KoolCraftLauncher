use tauri::App;

#[cfg(any(target_os = "macos", target_os = "windows"))]
use crate::window::vibrancy::*;
#[cfg(any(target_os = "macos", target_os = "windows"))]
use tauri::Manager;
#[cfg(target_os = "windows")]
use tauri_plugin_os::Version::Semantic;
#[cfg(target_os = "windows")]
use window_vibrancy::{apply_acrylic, apply_mica};

/// Returns whether the current Windows version can use acrylic without side effect.
#[cfg(target_os = "windows")]
pub fn can_use_acrylic(patch_version: u64) -> bool {
	const EARLIEST_PATCH_VER: u64 = 18090;
	const LATEST_PATCH_VER: u64 = 19039;
	(EARLIEST_PATCH_VER..=LATEST_PATCH_VER).contains(&patch_version)
}

pub fn is_win11(patch: u64) -> bool {
	const EARLIEST_WIN_11_PATCH_VERSION: u64 = 22000;
	patch >= EARLIEST_WIN_11_PATCH_VERSION
}

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
				if apply_mica(&win, Some(true)).is_ok() {
					return Ok(VibrancyState::Mica);
				}
			}
			Semantic(10, _, patch) => {
				win.set_shadow(false).unwrap();
				if can_use_acrylic(patch) && apply_acrylic(&win, Some((18, 18, 18, 125))).is_ok() {
					return Ok(VibrancyState::Acrylic);
				}
			}
			_ => {}
		}
	}

	Ok(VibrancyState::None)
}
