pub mod app_state;
pub mod commands;
pub mod controllers;
pub mod language;
pub mod settings;
pub mod setup;
pub mod toggle_shortcut;
pub mod plugins;
pub mod window_event;
use plugins::RegisterPlugins;
use commands::RegisterCommands;
use setup::AppSetup;
use window_event::ResiterWindowEvent;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .register_plugins()
        .register_commands()
        .register_window_event()
        .app_setup()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
