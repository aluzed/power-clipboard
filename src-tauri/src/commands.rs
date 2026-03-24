use serde::Serialize;
use tauri::{command, State};

use crate::clipboard_stack::{AppState, StackItem};

#[derive(Serialize)]
pub struct Settings {
    pub stack_limit: usize,
}

#[command]
pub fn get_stack_preview(state: State<'_, AppState>) -> Vec<StackItem> {
    let stack = state.stack.lock().unwrap();
    stack.items_preview()
}

#[command]
pub fn get_current_index(state: State<'_, AppState>) -> Option<usize> {
    let stack = state.stack.lock().unwrap();
    stack.current_index
}

#[command]
pub fn get_settings(state: State<'_, AppState>) -> Settings {
    let stack = state.stack.lock().unwrap();
    Settings {
        stack_limit: stack.max_size(),
    }
}

#[command]
pub fn set_stack_limit(state: State<'_, AppState>, limit: usize) {
    let mut stack = state.stack.lock().unwrap();
    stack.set_max_size(limit);
}
