use tao::{
    event::Event,
    event_loop::{ControlFlow, EventLoopBuilder},
};
use tray_icon::{
    menu::{
        Menu, MenuEvent, 
        MenuItem, PredefinedMenuItem
    }, 
    TrayIcon, TrayIconBuilder
};

mod helpers;
use helpers::load_icon;

enum UserEvent {
    MenuEvent(tray_icon::menu::MenuEvent),
}

fn main() {
    let light_icon_path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/light_icon.png");
    let light_icon = load_icon(std::path::Path::new(light_icon_path));

    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();
    MenuEvent::set_event_handler(Some(move |evnet| {
        let _ = proxy.send_event(UserEvent::MenuEvent(evnet));
    }));

    let tray_menu = Menu::new();
    let quit_item = MenuItem::new("Quit", true, None);
    let _ = tray_menu.append_items(&[
        &PredefinedMenuItem::separator(),
        &quit_item
    ]);

    let mut tray_icon: Option<TrayIcon> = None;

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
            }
            Event::UserEvent(UserEvent::MenuEvent(event)) => {
                if event.id == quit_item.id() {
                    tray_icon.take();
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => {}
        }
    })
}