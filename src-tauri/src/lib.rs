use tauri::plugin::TauriPlugin;
use tauri::{Builder, Manager, Wry};
use utils::window::setup_window;
use utils::window::vibrancy::VibrancyStateStore;
use crate::context::generate_context;

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
	let builder = add_plugins!(
		// tauri builder
		Builder::default(),
		// plugins
		#[cfg(not(mobile))]
		single_instance_plugin(),
		#[cfg(not(mobile))]
		tauri_plugin_window_state::Builder::default().build(),
		tauri_plugin_os::init(),
		tauri_plugin_http::init(),
		tauri_plugin_shell::init(),
		tauri_plugin_store::Builder::new().build(),
		#[cfg(not(mobile))]
		prevent_default_plugin(),
		log_plugin()
	)
	.manage(VibrancyStateStore::new())
	.setup(|app| {
		let vibrancy_state = setup_window::setup_window(app).unwrap();
		app.state::<VibrancyStateStore>().set(vibrancy_state);
		Ok(())
	});
	command::invoke_handler(builder)
		.run(generate_context())
		.expect("error while running tauri application");
}

fn log_plugin() -> TauriPlugin<Wry> {
	tauri_plugin_log::Builder::new()
		.level(log::LevelFilter::Info)
		.build()
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
