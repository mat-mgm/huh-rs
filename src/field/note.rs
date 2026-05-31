use bubbletea_rs::{Cmd, Msg};

use super::Field;

/// Static display-only note field.  Shows a title and body; advances on submit.
pub struct Note {
    title:   String,
    body:    String,
    focused: bool,
    key:     Option<String>,
}

impl Note {
    pub fn new() -> Self {
        Note {
            title:   String::new(),
            body:    String::new(),
            focused: false,
            key:     None,
        }
    }

    pub fn with_title(mut self, t: impl Into<String>) -> Self { self.title = t.into(); self }
    pub fn with_body(mut self, b: impl Into<String>) -> Self { self.body = b.into(); self }
    pub fn with_key(mut self, k: impl Into<String>) -> Self { self.key = Some(k.into()); self }
}

impl Default for Note {
    fn default() -> Self { Self::new() }
}

impl Field for Note {
    fn update(&mut self, _msg: &Msg) -> Option<Cmd> { None }

    fn view(&self) -> String {
        let mut out = String::new();
        if !self.title.is_empty() { out.push_str(&self.title); out.push('\n'); }
        if !self.body.is_empty()  { out.push_str(&self.body);  out.push('\n'); }
        out
    }

    fn focus(&mut self) -> Option<Cmd> { self.focused = true; None }
    fn blur(&mut self)  { self.focused = false; }
    fn is_done(&self) -> bool { !self.focused }
    fn validate(&self) -> Result<(), String> { Ok(()) }
    fn key(&self) -> Option<&str> { self.key.as_deref() }
}
