use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_clipboard_manager::ClipboardExt;

use crate::clipboard_stack::AppState;

pub fn start_clipboard_monitor(app: AppHandle) {
    std::thread::spawn(move || {
        let mut last_known = String::new();

        // Initialize with current clipboard content
        if let Ok(content) = app.clipboard().read_text() {
            last_known = content;
        }

        loop {
            std::thread::sleep(Duration::from_millis(500));

            let current = match app.clipboard().read_text() {
                Ok(text) => text,
                Err(_) => continue,
            };

            if current != last_known && !current.trim().is_empty() {
                last_known = current.clone();

                let state = app.state::<AppState>();
                let preview = {
                    let mut stack = state.stack.lock().unwrap();
                    stack.push(current);
                    stack.items_preview()
                };

                let _ = app.emit("clipboard-updated", preview);
            }
        }
    });
}
