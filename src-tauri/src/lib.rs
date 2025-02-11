use crate::context::generate_context;
use tauri::plugin::TauriPlugin;
use tauri::{App, Builder, LogicalPosition, Manager, WebviewBuilder, WebviewUrl, WindowBuilder, Wry};
use utils::window::setup_window;
use utils::window::vibrancy::VibrancyStateStore;

mod account;
mod command;
mod install;
mod task_manager;
mod utils;
mod context;

macro_rules! add_plugins {
    ($builder:expr, $($plugin:expr),*) => {
        {
            let mut builder: Builder<Wry> = $builder;
            $(
                builder = builder.plugin($plugin);
            )*
            builder
        }
    };
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	// This should be called as early in the execution of the app as possible
	#[cfg(debug_assertions)]
	let devtools = tauri_plugin_devtools::init();
	let builder = add_plugins!(
		// tauri builder
		Builder::default(),
		// plugins
		#[cfg(all(debug_assertions, not(mobile)))]
		devtools,
		#[cfg(not(mobile))]
		single_instance_plugin(),
		#[cfg(not(mobile))]
		tauri_plugin_window_state::Builder::default().build(),
		tauri_plugin_os::init(),
		tauri_plugin_http::init(),
		tauri_plugin_shell::init(),
		tauri_plugin_store::Builder::new().build(),
		#[cfg(not(mobile))]
		prevent_default_plugin()
	)
	.manage(VibrancyStateStore::new())
	.setup(|app| {
		#[cfg(debug_assertions)]
		open_vue_devtools(app);
		let vibrancy_state = setup_window::setup_window(app).unwrap();
		app.state::<VibrancyStateStore>().set(vibrancy_state);
		Ok(())
	});
	command::invoke_handler(builder)
		.run(generate_context())
		.expect("error while running tauri application");
}

fn prevent_default_plugin() -> TauriPlugin<Wry> {
	use tauri_plugin_prevent_default::Flags;
	let flags = Flags::all();

	#[cfg(debug_assertions)]
	let flags = flags.difference(Flags::DEV_TOOLS | Flags::RELOAD);

	tauri_plugin_prevent_default::Builder::new()
		.with_flags(flags)
		.build()
}

fn single_instance_plugin() -> TauriPlugin<Wry> {
	tauri_plugin_single_instance::init(|app, _args, _cwd| {
		let _ = app
			.get_webview_window("main")
			.expect("no main window")
			.set_focus();
	})
}

fn open_vue_devtools(app: &App) {
	let window_id = "vue-devtools";
	if let Ok(window) = WindowBuilder::new(app, window_id)
		.title("Vue Devtools")
		.build() {
		let webview_builder = WebviewBuilder::new(
			window_id,
			WebviewUrl::App("/__devtools__/".parse().unwrap()),
		);
		let _ = window.add_child(
			webview_builder,
			LogicalPosition::new(0, 0),
			window.inner_size().unwrap(),
		);
	}
}