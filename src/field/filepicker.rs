use bubbletea_rs::{Cmd, Msg};
use bubbles_rs::filepicker;

use super::Field;

/// File-picker field wrapping `bubbles_rs::filepicker::Model`.
pub struct FilePicker {
    title:          String,
    description:    String,
    inner:          filepicker::Model,
    selected_path:  Option<String>,
    focused:        bool,
    key:            Option<String>,
    allowed_types:  Vec<String>,
    dirs_only:      bool,
}

impl FilePicker {
    pub fn new() -> Self {
        FilePicker {
            title:         String::new(),
            description:   String::new(),
            inner:         filepicker::Model::new(),
            selected_path: None,
            focused:       false,
            key:           None,
            allowed_types: Vec::new(),
            dirs_only:     false,
        }
    }

    pub fn with_title(mut self, t: impl Into<String>) -> Self { self.title = t.into(); self }
    pub fn with_description(mut self, d: impl Into<String>) -> Self { self.description = d.into(); self }
    pub fn with_key(mut self, k: impl Into<String>) -> Self { self.key = Some(k.into()); self }
    pub fn with_dirs_only(mut self, v: bool) -> Self { self.dirs_only = v; self }
    pub fn with_allowed_types(mut self, types: Vec<String>) -> Self { self.allowed_types = types; self }

    pub fn value(&self) -> Option<&str> { self.selected_path.as_deref() }
}

impl Default for FilePicker {
    fn default() -> Self { Self::new() }
}

impl Field for FilePicker {
    fn update(&mut self, msg: &Msg) -> Option<Cmd> {
        if !self.focused { return None; }
        let cmd = self.inner.update(msg);
        let (selected, path) = self.inner.did_select_file(msg);
        if selected {
            self.selected_path = Some(path.to_string());
        }
        cmd
    }

    fn view(&self) -> String {
        let mut out = String::new();
        if !self.title.is_empty() { out.push_str(&self.title); out.push('\n'); }
        if !self.description.is_empty() { out.push_str(&self.description); out.push('\n'); }
        out.push_str(&self.inner.view());
        out
    }

    fn focus(&mut self) -> Option<Cmd> {
        self.focused = true;
        Some(self.inner.init())
    }

    fn blur(&mut self) { self.focused = false; }
    fn is_done(&self) -> bool { self.selected_path.is_some() }
    fn validate(&self) -> Result<(), String> {
        if self.selected_path.is_some() { Ok(()) }
        else { Err("no file selected".to_string()) }
    }
    fn key(&self) -> Option<&str> { self.key.as_deref() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_state_not_done() {
        let fp = FilePicker::new();
        assert!(!fp.is_done());
        assert!(fp.value().is_none());
    }

    #[test]
    fn validate_without_selection_is_err() {
        let fp = FilePicker::new();
        assert!(fp.validate().is_err());
    }

    #[test]
    fn view_shows_title() {
        let fp = FilePicker::new().with_title("Pick file");
        assert!(fp.view().contains("Pick file"));
    }

    #[test]
    fn focus_sets_focused() {
        let mut fp = FilePicker::new();
        fp.focus();
        assert!(fp.focused);
        fp.blur();
        assert!(!fp.focused);
    }
}
