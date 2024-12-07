use serde::{Deserialize, Serialize};

use crate::toggle_shortcut::ToggleShortcut;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub run_on_startup: bool,
    pub hide_to_tray: bool,
    pub toggle_shortcut: ToggleShortcut,
}

pub const DEFAULT_SETTINGS: Settings = Settings {
    run_on_startup: false,
    hide_to_tray: false,
    toggle_shortcut: ToggleShortcut::CtrlAlt,
};