use std::sync::mpsc::{self, Receiver};
use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem},
    TrayIcon, TrayIconBuilder,
};

/// Message types that can be sent from the tray
pub enum TrayMessage {
    Exit,
}

/// Icon bytes embedded at compile time
const ICON_BYTES: &[u8] = include_bytes!("../assets/icon.png");

/// Initialize platform-specific requirements
fn platform_init() -> Result<(), Box<dyn std::error::Error>> {
    // GTK must be initialized for tray menus on Linux.
    // The gtk crate is already a transitive dependency of tray-icon,
    // so we're not adding any new dependencies here.
    #[cfg(target_os = "linux")]
    gtk::init()?;

    Ok(())
}

/// Create the system tray icon and menu
/// Returns the tray icon (must be kept alive) and a receiver for exit messages
pub fn create_tray() -> Result<(TrayIcon, Receiver<TrayMessage>), Box<dyn std::error::Error>> {
    // Initialize platform-specific requirements
    platform_init()?;

    // Load icon from embedded bytes
    let icon_image = image::load_from_memory(ICON_BYTES)?;
    let icon_rgba = icon_image.to_rgba8();
    let (width, height) = icon_rgba.dimensions();
    let icon = tray_icon::Icon::from_rgba(icon_rgba.into_raw(), width, height)?;

    // Create menu
    let menu = Menu::new();
    let exit_item = MenuItem::new("Exit Omega Tab", true, None);
    menu.append(&exit_item)?;

    // Create tray icon
    let tray = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip("Omega Tab")
        .with_icon(icon)
        .build()?;

    // Create channel for exit messages
    let (tx, rx) = mpsc::channel();

    // Listen for menu events in a separate thread
    let exit_id = exit_item.id().clone();
    std::thread::spawn(move || loop {
        if let Ok(event) = MenuEvent::receiver().recv() {
            if event.id == exit_id {
                let _ = tx.send(TrayMessage::Exit);
                break;
            }
        }
    });

    Ok((tray, rx))
}
