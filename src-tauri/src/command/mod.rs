use tauri::{Builder, Wry};
use crate::command::window::*;

mod window;

pub fn invoke_handler(builder: Builder<Wry>) -> Builder<Wry> {
	builder.invoke_handler(tauri::generate_handler![
		// windows commands
		get_vibrancy_state,
		should_custom_window,
	])
}