use tao::{
    event::Event,
    event_loop::{ControlFlow, EventLoopBuilder},
};
use tray_icon::{
    menu::{
        CheckMenuItem, Menu, MenuEvent, 
        MenuItem, PredefinedMenuItem, Submenu
    }, 
    TrayIcon, TrayIconBuilder
};
use keepawake::KeepAwake;

mod types;
mod keepawake;
mod helpers;
mod preferences;

use preferences::{
    Preferences, 
    DEACTIVE_ON_LOW_BATTERY_PREFERENCE, 
    PREVENT_SCREEN_DIMMING_PREFERENCE, 
    PREVENT_SLEEPING_PREFERENCE
};
use types::Options;
enum UserEvent {
    MenuEvent(tray_icon::menu::MenuEvent),
}

fn main() {
    let (light_icon, dark_icon) = helpers::get_icons();

    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();
    MenuEvent::set_event_handler(Some(move |evnet| {
        let _ = proxy.send_event(UserEvent::MenuEvent(evnet));
    }));

    let preferences_submenu = Submenu::new("Preferences", true);
    let prevent_screen_dimming_item = CheckMenuItem::new("Prevent screen dimming", true, true, None);
    let prevent_sleeping_item = CheckMenuItem::new("Prevent sleeping", true, true, None);
    let deactivate_on_low_battery_item = CheckMenuItem::new("Deactivate on low battery", true, true, None);
    let _ = preferences_submenu.append_items(&[
        &prevent_screen_dimming_item,
        &prevent_sleeping_item,
        &PredefinedMenuItem::separator(),
        &deactivate_on_low_battery_item
    ]);

    let tray_menu = Menu::new();
    let activate_item = MenuItem::new("Activate", true, None);
    let quit_item = MenuItem::new("Quit", true, None);
    let _ = tray_menu.append_items(&[
        &activate_item,
        &PredefinedMenuItem::separator(),
        &preferences_submenu,
        &PredefinedMenuItem::separator(),
        &quit_item
    ]);

    let mut tray_icon: Option<TrayIcon> = None;
    let mut keepawake: Option<KeepAwake> = None;
    let mut preferences: Option<Preferences> = None;
    let mut is_activated: bool = false;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(tao::event::StartCause::Init) => {
                tray_icon = Some(
                    TrayIconBuilder::new()
                        .with_menu(Box::new(tray_menu.clone()))
                        .with_icon(light_icon.clone())
                        .build()
                        .unwrap()
                );
                keepawake = Some(KeepAwake::new(None).unwrap());
                preferences = Some(Preferences::new());
                
                preferences.as_ref().unwrap().set_initial_values();
                prevent_screen_dimming_item.set_checked(
                    preferences.as_ref().unwrap().load_preference(PREVENT_SCREEN_DIMMING_PREFERENCE)
                );
                prevent_sleeping_item.set_checked(
                    preferences.as_ref().unwrap().load_preference(PREVENT_SLEEPING_PREFERENCE)
                );
                deactivate_on_low_battery_item.set_checked(
                    preferences.as_ref().unwrap().load_preference(DEACTIVE_ON_LOW_BATTERY_PREFERENCE)
                );

                helpers::listen_for_theme_changes(
                    tray_icon.as_ref().unwrap().clone(), 
                    light_icon.clone(), 
                    dark_icon.clone()
                );
            }
            Event::UserEvent(UserEvent::MenuEvent(event)) => {
                if event.id == activate_item.id() {
                    if is_activated {
                        drop(keepawake.clone());

                        is_activated = false;
                        activate_item.set_text("Activate");
                    } else {
                        keepawake.as_mut().unwrap().set_options(Options::from_preferences(preferences.as_ref().unwrap()));

                        if keepawake.as_mut().cloned().unwrap().activate().is_ok() {
                            is_activated = true;
                            activate_item.set_text("Deactivate");
                        }
                    }
                }

                if event.id == prevent_screen_dimming_item.id() && is_activated || 
                    event.id == prevent_sleeping_item.id() && is_activated {
                        drop(keepawake.clone());

                        keepawake.as_mut().unwrap().set_options(Options::from_preferences(preferences.as_ref().unwrap()));
                        let _ = keepawake.as_mut().cloned().unwrap().activate();
                    }

                if event.id == prevent_screen_dimming_item.id() {
                    if !prevent_sleeping_item.is_checked() {
                        prevent_screen_dimming_item.set_checked(true);
                    } else {
                        preferences.as_ref().unwrap().toggle_preference(PREVENT_SCREEN_DIMMING_PREFERENCE);
                        prevent_screen_dimming_item.set_checked(
                            preferences.as_ref().unwrap().load_preference(PREVENT_SCREEN_DIMMING_PREFERENCE)
                        );
                    }
                }

                if event.id == prevent_sleeping_item.id() {
                    if !prevent_screen_dimming_item.is_checked() {
                        prevent_sleeping_item.set_checked(true);
                    } else {
                        preferences.as_ref().unwrap().toggle_preference(PREVENT_SLEEPING_PREFERENCE);
                        prevent_sleeping_item.set_checked(
                            preferences.as_ref().unwrap().load_preference(PREVENT_SLEEPING_PREFERENCE)
                        );
                    }
                }

                if event.id == deactivate_on_low_battery_item.id() {
                    preferences.as_ref().unwrap().toggle_preference(DEACTIVE_ON_LOW_BATTERY_PREFERENCE);
                    deactivate_on_low_battery_item.set_checked(
                        preferences.as_ref().unwrap().load_preference(DEACTIVE_ON_LOW_BATTERY_PREFERENCE)
                    );
                }

                if event.id == quit_item.id() {
                    tray_icon.take();
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => {}
        }
    })
}