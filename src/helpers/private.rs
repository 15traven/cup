use windows_registry::CURRENT_USER;
use super::ColorMode;

const SUBKEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize";
const VALUE: &str = "AppsUseLightTheme";

pub fn get_system_theme() -> ColorMode {
    let key = CURRENT_USER.open(SUBKEY);
    let value = key.unwrap().get_value(VALUE).unwrap();

    value[0].into()
}

pub fn load_icon(path: &std::path::Path) -> tray_icon::Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    tray_icon::Icon::from_rgba(
        icon_rgba, 
        icon_width, 
        icon_height
    ).expect("Failed to open icon")
}
