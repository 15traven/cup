use std::{
    thread::{self, sleep},
    time::Duration
};

use tray_icon::{TrayIcon, Icon};

use super::get_system_theme;

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
                    super::ColorMode::Dark => tray_icon.set_icon(Some(light_icon.clone())),
                    super::ColorMode::Light => tray_icon.set_icon(Some(dark_icon.clone())),
                    super::ColorMode::Unspecified => todo!(),
                };

                prev_theme = current_theme;
            }

            sleep(Duration::from_secs(5));
        }
    });
}