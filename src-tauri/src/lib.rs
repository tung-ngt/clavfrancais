use std::sync::Mutex;

use tauri::{
    image::Image,
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent, TrayIconId},
    Error, Manager, WindowEvent,
};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
const TRAY_ICON_ID: &str = "10";

#[derive(Debug, PartialEq, Eq)]
enum Language {
    English,
    French,
}
struct AppState {
    language: Language,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppState {
                language: Language::English,
            }));

            let icon_image = Image::from_path("./icons/france.png").expect("must have icon file");
            let quit = MenuItemBuilder::new("Quit").id("quit").build(app).unwrap();
            let show = MenuItemBuilder::new("Show").id("show").build(app).unwrap();
            let toggle = MenuItemBuilder::new("Toggle")
                .id("toggle")
                .build(app)
                .unwrap();
            let menu = MenuBuilder::new(app)
                .items(&[&quit, &show, &toggle])
                .build()
                .unwrap();
            let _tray = TrayIconBuilder::with_id(TRAY_ICON_ID)
                .icon(icon_image)
                .menu(&menu)
                .menu_on_left_click(false)
                .on_menu_event(|app_handle, menu_event| match menu_event.id().as_ref() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    "show" => {
                        if let Some(window) = app_handle.get_webview_window("main") {
                            let _ = window.show();
                        }
                    }
                    "toggle" => {
                        let app_state = app_handle.state::<Mutex<AppState>>();
                        let mut app_state = app_state.lock().unwrap();

                        let icon_image = if app_state.language == Language::French {
                            app_state.language = Language::English;
                            Image::from_path("./icons/uk.png").expect("must have icon file")
                        } else {
                            app_state.language = Language::French;
                            Image::from_path("./icons/france.png").expect("must have icon file")
                        };
                        if let Some(tray_icon) =
                            app_handle.tray_by_id(&TrayIconId::new(TRAY_ICON_ID))
                        {
                            let _result = tray_icon.set_icon(Some(icon_image));
                        }
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray_icon, event| {
                    if let TrayIconEvent::Click {
                        button,
                        button_state,
                        ..
                    } = event
                    {
                        if (button != MouseButton::Left) | (button_state != MouseButtonState::Up) {
                            return;
                        }
                        let app_handle = tray_icon.app_handle();
                        let app_state = app_handle.state::<Mutex<AppState>>();
                        let mut app_state = app_state.lock().unwrap();

                        let icon_image = if app_state.language == Language::French {
                            app_state.language = Language::English;
                            Image::from_path("./icons/uk.png").expect("must have icon file")
                        } else {
                            app_state.language = Language::French;
                            Image::from_path("./icons/france.png").expect("must have icon file")
                        };

                        let _result = tray_icon.set_icon(Some(icon_image));
                    }
                })
                .build(app)?;

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                let _ = window.hide();
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
