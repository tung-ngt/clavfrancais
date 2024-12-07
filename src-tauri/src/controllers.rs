use std::{fs, path::PathBuf, sync::Mutex, thread};
use clavfrancais_engine::{char_buffer::StackSizedCharBuffer, engine::Engine, input_controller::setup_key_combination_map};
use tauri::{image::Image, tray::TrayIconId, AppHandle, Emitter, Manager};
use tauri_plugin_autostart::ManagerExt;

use crate::{app_state::AppState, language::Language, settings::{Settings, DEFAULT_SETTINGS}, toggle_shortcut::ToggleShortcut};

pub fn change_language(app_handle: &AppHandle, language: Language) {
    let app_state = app_handle.state::<Mutex<AppState>>();
    let mut app_state = app_state.lock().unwrap();

    let icon_image = if language == Language::English {
        app_state.language = Language::English;
        let _ = app_handle.emit("change_language", Language::English);
        Image::from_path("resources/uk.png").expect("must have icon file")
    } else {
        app_state.language = Language::French;
        let _ = app_handle.emit("change_language", Language::French);
        Image::from_path("resources/france.png").expect("must have icon file")
    };

    if let Some(tray_icon) = app_handle.tray_by_id(&TrayIconId::new(TRAY_ICON_ID)) {
        let _ = tray_icon.set_icon(Some(icon_image));
    }

    if language == Language::French {
        start_engine();
    } else {
        stop_engine();
    }
}


pub fn toggle_language(app_handle: &AppHandle) {
    let new_language = {
        let app_state = app_handle.state::<Mutex<AppState>>();
        let app_state = app_state.lock().unwrap();
        if app_state.language == Language::English {
            Language::French
        } else {
            Language::English
        }
    };
    change_language(app_handle, new_language);
}

pub const TRAY_ICON_ID: &str = "10";

pub fn start_engine() {
    thread::spawn(|| {
        Engine::start(
            setup_key_combination_map(),
            StackSizedCharBuffer::<30>::default(),
        );
    });
}

pub fn stop_engine() {
    Engine::stop();
}

pub fn quit() {
    std::process::exit(0);
}

pub fn handle_save_settings(app: &AppHandle, path: PathBuf, settings: Settings) {
    let settings_json = serde_json::to_string(&settings).unwrap();
    let r = fs::write(path, settings_json);
    println!("{:?}", r);

    if settings.run_on_startup {
        enable_run_on_startup(app);
    } else {
        disable_run_on_startup(app);
    }
}

pub fn handle_load_settings(path: PathBuf) -> Settings {
    if !path.is_file() {
        let settings_json = serde_json::to_string(&DEFAULT_SETTINGS).unwrap();
        let _ = fs::write(&path, settings_json);
        return DEFAULT_SETTINGS;
    }

    let settings_json = fs::read_to_string(&path).unwrap();
    let r = serde_json::from_str(&settings_json);

    if let Ok(settings) = r {
        return settings;
    }

    let settings_json = serde_json::to_string(&DEFAULT_SETTINGS).unwrap();
    let _ = fs::write(&path, settings_json);
    DEFAULT_SETTINGS
}

pub fn enable_run_on_startup(app: &AppHandle) {
    let auto_start_manager = app.autolaunch();
    if auto_start_manager.is_enabled().unwrap() {
        return;
    }
    let r = auto_start_manager.enable();
    println!("{:?}", r);
}

pub fn disable_run_on_startup(app: &AppHandle) {
    let auto_start_manager = app.autolaunch();
    if !auto_start_manager.is_enabled().unwrap() {
        return;
    }
    let r = auto_start_manager.disable();
    println!("{:?}", r);
}

pub fn handle_shortcut(app: &AppHandle, toggle_type: ToggleShortcut) {
    let app_state = app.state::<Mutex<AppState>>();
    let app_state = app_state.lock().unwrap();
    if app_state.settings.toggle_shortcut == toggle_type {
        toggle_language(app);
    }
}
