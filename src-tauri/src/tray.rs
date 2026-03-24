use tauri::{
    image::Image,
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};

#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;

pub fn create_tray(app: &AppHandle) -> tauri::Result<()> {
    // Hide from dock by default (macOS)
    #[cfg(target_os = "macos")]
    let _ = app.set_activation_policy(ActivationPolicy::Accessory);

    let settings_item = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&settings_item, &quit_item])?;

    TrayIconBuilder::new()
        .icon(
            Image::from_bytes(include_bytes!("../icons/tray-icon.png"))
                .expect("Failed to load tray icon"),
        )
        .icon_as_template(true)
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "settings" => {
                show_settings_window(app);
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|_tray, _event| {})
        .build(app)?;

    Ok(())
}

pub fn show_settings_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("settings") {
        // Show dock icon while settings are open (macOS)
        #[cfg(target_os = "macos")]
        let _ = app.set_activation_policy(ActivationPolicy::Regular);

        let _ = window.show();
        let _ = window.set_focus();

        // Listen for window close to hide dock icon again
        let app_handle = app.clone();
        window.on_window_event(move |event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                // Hide instead of closing so we can reuse it
                api.prevent_close();
                if let Some(w) = app_handle.get_webview_window("settings") {
                    let _ = w.hide();
                }
                #[cfg(target_os = "macos")]
                let _ = app_handle.set_activation_policy(ActivationPolicy::Accessory);
            }
        });
    }
}
