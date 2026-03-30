use serde::Serialize;
use std::collections::VecDeque;
use std::sync::Mutex;

#[derive(Debug, Serialize, Clone)]
pub struct StackItem {
    pub index: usize,
    pub preview: String,
    pub full_text: String,
}

pub struct ClipboardStack {
    items: VecDeque<String>,
    max_size: usize,
    pub current_index: Option<usize>,
    pub navigating: bool,
}

impl ClipboardStack {
    pub fn new(max_size: usize) -> Self {
        Self {
            items: VecDeque::new(),
            max_size,
            current_index: None,
            navigating: false,
        }
    }

    pub fn push(&mut self, value: String) {
        if value.trim().is_empty() {
            return;
        }
        // Remove duplicate if exists
        if let Some(pos) = self.items.iter().position(|item| item == &value) {
            self.items.remove(pos);
        }
        // Push to front
        self.items.push_front(value);
        // Trim to max size
        while self.items.len() > self.max_size {
            self.items.pop_back();
        }
        // Reset navigation index
        self.current_index = None;
    }

    pub fn navigate_up(&mut self) -> Option<&String> {
        if self.items.is_empty() {
            return None;
        }
        self.navigating = true;
        match self.current_index {
            None => {
                self.current_index = Some(0);
            }
            Some(idx) => {
                if idx > 0 {
                    self.current_index = Some(idx - 1);
                }
            }
        }
        self.current()
    }

    pub fn navigate_down(&mut self) -> Option<&String> {
        if self.items.is_empty() {
            return None;
        }
        self.navigating = true;
        match self.current_index {
            None => {
                // First press: skip index 0 (current clipboard) and go to index 1
                if self.items.len() > 1 {
                    self.current_index = Some(1);
                } else {
                    self.current_index = Some(0);
                }
            }
            Some(idx) => {
                if idx + 1 < self.items.len() {
                    self.current_index = Some(idx + 1);
                }
            }
        }
        self.current()
    }

    pub fn current(&self) -> Option<&String> {
        self.current_index
            .and_then(|idx| self.items.get(idx))
    }

    pub fn reset_index(&mut self) {
        self.current_index = None;
        self.navigating = false;
    }

    pub fn set_max_size(&mut self, n: usize) {
        self.max_size = n;
        while self.items.len() > self.max_size {
            self.items.pop_back();
        }
    }

    pub fn max_size(&self) -> usize {
        self.max_size
    }

    fn make_preview(text: &str) -> String {
        let trimmed = text.trim();
        let chars: Vec<char> = trimmed.chars().collect();
        if chars.len() <= 2 {
            trimmed.to_string()
        } else {
            format!("{}…", chars[..2].iter().collect::<String>())
        }
    }

    pub fn items_preview(&self) -> Vec<StackItem> {
        self.items
            .iter()
            .enumerate()
            .map(|(index, text)| StackItem {
                index,
                preview: Self::make_preview(text),
                full_text: text.clone(),
            })
            .collect()
    }
}

pub struct AppState {
    pub stack: Mutex<ClipboardStack>,
}

impl AppState {
    pub fn new(max_size: usize) -> Self {
        Self {
            stack: Mutex::new(ClipboardStack::new(max_size)),
        }
    }
}
