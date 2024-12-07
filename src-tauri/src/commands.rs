use std::sync::Mutex;

use tauri::{AppHandle, Builder, Manager, Wry};

use crate::{
    app_state::AppState,
    controllers::{change_language, handle_save_settings, quit, toggle_language},
    language::Language,
    settings::Settings,
};

#[tauri::command]
pub fn get_language_command(app: AppHandle) -> Language {
    let app_state = app.state::<Mutex<AppState>>();

    let language = if let Ok(app_state) = app_state.lock() {
        app_state.language
    } else {
        Language::English
    };

    language
}

#[tauri::command]
pub fn change_language_command(app_handle: AppHandle, language: Language) {
    change_language(&app_handle, language);
}

#[tauri::command]
pub fn toggle_language_command(app_handle: AppHandle) {
    toggle_language(&app_handle);
}

#[tauri::command]
pub fn quit_command() {
    quit();
}

#[tauri::command]
pub fn get_settings_command(app_handle: AppHandle) -> Settings {
    let app_state = app_handle.state::<Mutex<AppState>>();
    let app_state = app_state.lock().unwrap();
    app_state.settings
}

#[tauri::command]
pub fn set_settings_command(app_handle: AppHandle, settings: Settings) {
    let app_state = app_handle.state::<Mutex<AppState>>();
    let mut app_state = app_state.lock().unwrap();
    app_state.settings = settings;
    handle_save_settings(
        &app_handle,
        app_handle.path().app_config_dir().unwrap(),
        settings,
    );
}

pub trait RegisterCommands {
    fn register_commands(self) -> Self;
}

impl RegisterCommands for Builder<Wry> {
    fn register_commands(self) -> Self {
        self.invoke_handler(tauri::generate_handler![
            get_language_command,
            change_language_command,
            quit_command,
            get_settings_command,
            set_settings_command,
            toggle_language_command
        ])
    }
}
