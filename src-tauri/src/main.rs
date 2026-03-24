// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod clipboard_monitor;
mod clipboard_stack;
mod commands;
mod tray;

use clipboard_stack::AppState;
use serde::Serialize;
use std::sync::atomic::{AtomicU64, Ordering};
use tauri::{Emitter, Manager};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

#[derive(Clone, Serialize)]
struct OverlayPayload {
    items: Vec<clipboard_stack::StackItem>,
    current_index: Option<usize>,
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, shortcut, event| {
                    if event.state != ShortcutState::Pressed {
                        return;
                    }

                    let shortcut_str = shortcut.to_string();
                    let is_up = shortcut_str.contains("Up");
                    let is_down = shortcut_str.contains("Down");

                    if !is_up && !is_down {
                        return;
                    }

                    let state = app.state::<AppState>();
                    let (items, current_index, selected_text) = {
                        let mut stack = state.stack.lock().unwrap();
                        if is_up {
                            stack.navigate_up();
                        } else {
                            stack.navigate_down();
                        }
                        let selected = stack.current().cloned();
                        (stack.items_preview(), stack.current_index, selected)
                    };

                    // Write selected item to clipboard
                    if let Some(text) = selected_text {
                        let _ = app.clipboard().write_text(&text);
                    }

                    // Show overlay
                    if let Some(overlay) = app.get_webview_window("overlay") {
                        // Position top-right of primary monitor
                        if let Ok(Some(monitor)) = app.primary_monitor() {
                            let screen_size = monitor.size();
                            let scale = monitor.scale_factor();
                            let x = (screen_size.width as f64 / scale) - 300.0;
                            let y = 40.0;
                            let _ = overlay.set_position(tauri::LogicalPosition::new(x, y));
                        }
                        let _ = overlay.show();
                    }

                    // Emit navigation event
                    let payload = OverlayPayload {
                        items,
                        current_index,
                    };
                    let _ = app.emit("stack-navigated", payload);

                    // Debounced auto-hide: each navigation bumps the counter.
                    // The timer only hides if no new navigation happened since it was spawned.
                    static NAV_COUNTER: AtomicU64 = AtomicU64::new(0);
                    let nav_id = NAV_COUNTER.fetch_add(1, Ordering::SeqCst) + 1;

                    let app_handle = app.clone();
                    std::thread::spawn(move || {
                        std::thread::sleep(std::time::Duration::from_secs(2));
                        // Only hide if no new navigation happened during the wait
                        if NAV_COUNTER.load(Ordering::SeqCst) == nav_id {
                            if let Some(overlay) = app_handle.get_webview_window("overlay") {
                                let _ = overlay.hide();
                            }
                            let state = app_handle.state::<AppState>();
                            let mut stack = state.stack.lock().unwrap();
                            stack.reset_index();
                        }
                    });
                })
                .build(),
        )
        .setup(|app| {
            // Initialize app state with default stack size
            let state = AppState::new(5);
            app.manage(state);

            // Register global shortcuts
            let app_handle = app.handle();
            let shortcut_up: Shortcut = "CommandOrControl+Shift+ArrowUp".parse().unwrap();
            let shortcut_down: Shortcut = "CommandOrControl+Shift+ArrowDown".parse().unwrap();
            app_handle.global_shortcut().register(shortcut_up)?;
            app_handle.global_shortcut().register(shortcut_down)?;

            // Create system tray
            tray::create_tray(app_handle)?;

            // Start clipboard monitor
            clipboard_monitor::start_clipboard_monitor(app_handle.clone());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_stack_preview,
            commands::get_current_index,
            commands::get_settings,
            commands::set_stack_limit,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Power Clipboard");
}
