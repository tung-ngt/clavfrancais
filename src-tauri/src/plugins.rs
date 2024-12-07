use tauri::{Builder, Wry};

use tauri_plugin_autostart::MacosLauncher;


pub trait RegisterPlugins {
    fn register_plugins(self) -> Self;
}

impl RegisterPlugins for Builder<Wry> {
    fn register_plugins(self) -> Self {
        self
            .plugin(tauri_plugin_autostart::init(
                MacosLauncher::LaunchAgent,
                None,
            ))
            .plugin(tauri_plugin_single_instance::init(|_, _, _| {}))
            .plugin(tauri_plugin_shell::init())
    }
}

