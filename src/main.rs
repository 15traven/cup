use keepawake::{KeepAwake, Options};
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

mod keepawake;
mod helpers;

enum UserEvent {
    MenuEvent(tray_icon::menu::MenuEvent),
}

fn main() {
    let light_icon_path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/light_icon.png");
    let light_icon = helpers::load_icon(std::path::Path::new(light_icon_path));

    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();
    MenuEvent::set_event_handler(Some(move |evnet| {
        let _ = proxy.send_event(UserEvent::MenuEvent(evnet));
    }));

    let preferences_submenu = Submenu::new("Preferences", true);
    let prevent_screen_dimming_item = CheckMenuItem::new(
        "Prevent screen dimming",
        true,
        true,
        None
    );
    let prevent_sleeping_item = CheckMenuItem::new(
        "Prevent sleeping", 
        true,
        true,
        None
    );
    let _ = preferences_submenu.append_items(&[
        &prevent_screen_dimming_item,
        &prevent_sleeping_item
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
            }
            Event::UserEvent(UserEvent::MenuEvent(event)) => {
                if event.id == activate_item.id() {
                    if is_activated {
                        drop(keepawake.clone());

                        is_activated = false;
                        activate_item.set_text("Activate");
                    } else {
                        keepawake.as_mut().unwrap().set_options(Options {
                            display: prevent_screen_dimming_item.is_checked(),
                            idle: prevent_sleeping_item.is_checked(),
                        });

                        if keepawake.as_mut().unwrap().activate().is_ok() {
                            is_activated = true;
                            activate_item.set_text("Deactivate");
                        }
                    }
                }

                if event.id == prevent_screen_dimming_item.id() && is_activated || 
                    event.id == prevent_sleeping_item.id() && is_activated {
                        drop(keepawake.clone());

                        keepawake.as_mut().unwrap().set_options(Options {
                            display: prevent_screen_dimming_item.is_checked(),
                            idle: prevent_sleeping_item.is_checked()
                        });
                        let _ = keepawake.as_mut().unwrap().activate();
                    }

                if event.id == prevent_screen_dimming_item.id() {
                    if !prevent_sleeping_item.is_checked() {
                        prevent_screen_dimming_item.set_checked(true);
                    }
                }

                if event.id == prevent_sleeping_item.id() {
                    if !prevent_screen_dimming_item.is_checked() {
                        prevent_sleeping_item.set_checked(true);
                    }
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