use windows_registry::CURRENT_USER;
use super::ColorMode;

const SUBKEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize";
const VALUE: &str = "AppsUseLightTheme";

pub fn get_system_theme() -> ColorMode {
    let key = CURRENT_USER.open(SUBKEY);
    let value = key.unwrap().get_value(VALUE).unwrap();

    value[0].into()
}