use std::{
    thread::{self, sleep},
    time::Duration
};
use tray_icon::{TrayIcon, Icon};
use crate::types::ColorMode;
use super::{get_system_theme, load_icon};

pub fn get_icons() -> (Icon, Icon) {
    let light_icon_path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/light_icon.png");
    let light_icon = load_icon(std::path::Path::new(light_icon_path));

    let dark_icon_path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/dark_icon.png");
    let dark_icon = load_icon(std::path::Path::new(dark_icon_path));

    (light_icon, dark_icon)
}

pub fn listen_for_theme_changes(
    mut tray_icon: TrayIcon,
    light_icon: Icon,
    dark_icon: Icon
) {
    thread::spawn(move || {
        let mut prev_theme = get_system_theme();

        loop {
            let current_theme = get_system_theme();
            if prev_theme != current_theme {
                let _ = match current_theme {
                    ColorMode::Dark => tray_icon.set_icon(Some(light_icon.clone())),
                    ColorMode::Light => tray_icon.set_icon(Some(dark_icon.clone())),
                    ColorMode::Unspecified => todo!(),
                };

                prev_theme = current_theme;
            }

            sleep(Duration::from_secs(5));
        }
    });
}