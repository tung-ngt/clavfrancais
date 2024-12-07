use std::error::Error;

use crate::{
    app_state::AppState,
    controllers::{handle_load_settings, quit, toggle_language, TRAY_ICON_ID},
    language::Language,
};
use clavfrancais_engine::engine::Engine;

use std::sync::{mpsc, Mutex};
use std::thread;
use tauri::{App, Builder, Wry};
use tauri::{
    image::Image,
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};


pub trait AppSetup {
    fn app_setup(self)  -> Self;
}

impl AppSetup for Builder<Wry> {
   fn app_setup(self)  -> Self {
       self.setup(setup)
   } 
}

fn setup(app: &mut App) -> Result<(), Box<dyn Error>> {
    let settings_path = app.path().app_config_dir().unwrap();
    let settings = handle_load_settings(settings_path);

    if !settings.hide_to_tray {
        let _ = app.get_webview_window("main").unwrap().show();
    }

    let app_handle = app.app_handle().clone();

    thread::spawn(move || {
        let (sender, receiver) = mpsc::channel::<()>();
        Engine::set_toggle_channel(sender);

        loop {
            let r = receiver.recv();
            if r.is_err() {
                break;
            }
            println!("yo");
            toggle_language(&app_handle);
        }
    });

    app.manage(Mutex::new(AppState {
        language: Language::English,
        settings,
    }));

    let icon_image = Image::from_path("resources/uk.png").expect("must have icon file");
    let quit_item = MenuItemBuilder::new("Quit").id("quit").build(app).unwrap();
    let show_item = MenuItemBuilder::new("Show").id("show").build(app).unwrap();
    let toggle_item = MenuItemBuilder::new("Toggle")
        .id("toggle")
        .build(app)
        .unwrap();
    let menu = MenuBuilder::new(app)
        .items(&[&quit_item, &show_item, &toggle_item])
        .build()
        .unwrap();
    let _tray = TrayIconBuilder::with_id(TRAY_ICON_ID)
        .icon(icon_image)
        .menu(&menu)
        .menu_on_left_click(false)
        .on_menu_event(|app_handle, menu_event| match menu_event.id().as_ref() {
            "quit" => {
                quit();
            }
            "show" => {
                if let Some(window) = app_handle.get_webview_window("main") {
                    let _ = window.show();
                }
            }
            "toggle" => {
                toggle_language(app_handle);
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
                toggle_language(tray_icon.app_handle());
            }
        })
        .build(app)?;

    Ok(())
}