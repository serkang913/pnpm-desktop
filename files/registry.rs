//! Windows registry read helpers for Pnpm Desktop.

pub const APP_KEY: &str = "Software\\Pnpm\\Pnpm Desktop";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Pnpm Desktop";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Pnpm Desktop") },
        RegValue { name: "Version",     value: String::from("v4.7.13") },
        RegValue { name: "Channel",     value: String::from("stable") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Pnpm Desktop") },
        RegValue { name: "Publisher",       value: String::from("Pnpm") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Pnpm Desktop\\uninstall.exe\"") },
    ]
}
