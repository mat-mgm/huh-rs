use bubbletea_rs::{Cmd, Msg};
use bubbles_rs::textarea;

use crate::validate::ValidateFunc;
use super::Field;

/// Multi-line textarea field.
pub struct Text {
    title:       String,
    description: String,
    inner:       textarea::Model,
    validate:    ValidateFunc<String>,
    err:         Option<String>,
    focused:     bool,
    key:         Option<String>,
}

impl Text {
    pub fn new() -> Self {
        Text {
            title:       String::new(),
            description: String::new(),
            inner:       textarea::Model::new(),
            validate:    crate::validate::no_op(),
            err:         None,
            focused:     false,
            key:         None,
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    pub fn with_key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    pub fn with_width(mut self, w: usize) -> Self {
        self.inner.set_width(w);
        self
    }

    pub fn with_validate<F: Fn(&String) -> Result<(), String> + Send + 'static>(
        mut self,
        f: F,
    ) -> Self {
        self.validate = Box::new(f);
        self
    }

    pub fn value(&self) -> String {
        self.inner.value()
    }
}

impl Default for Text {
    fn default() -> Self {
        Self::new()
    }
}

impl Field for Text {
    fn update(&mut self, msg: &Msg) -> Option<Cmd> {
        if !self.focused { return None; }
        self.inner.update(msg)
    }

    fn view(&self) -> String {
        let mut out = String::new();
        if !self.title.is_empty() {
            out.push_str(&self.title);
            out.push('\n');
        }
        if !self.description.is_empty() {
            out.push_str(&self.description);
            out.push('\n');
        }
        out.push_str(&self.inner.view());
        if let Some(e) = &self.err {
            out.push('\n');
            out.push_str(e);
        }
        out
    }

    fn focus(&mut self) -> Option<Cmd> {
        self.focused = true;
        self.inner.focus()
    }

    fn blur(&mut self) {
        self.focused = false;
        self.inner.blur();
    }

    fn is_done(&self) -> bool {
        !self.focused
    }

    fn validate(&self) -> Result<(), String> {
        let v = self.inner.value();
        (self.validate)(&v)
    }

    fn key(&self) -> Option<&str> {
        self.key.as_deref()
    }

    fn run_accessible(&mut self) -> Result<(), String> {
        use std::io::{self, BufRead, Write};
        let prompt = if self.title.is_empty() { "Text" } else { &self.title };
        println!("{} (enter blank line to finish):", prompt);
        let stdin = io::stdin();
        let mut buf = String::new();
        for line in stdin.lock().lines() {
            let l = line.map_err(|e| e.to_string())?;
            if l.is_empty() { break; }
            if !buf.is_empty() { buf.push('\n'); }
            buf.push_str(&l);
        }
        let _ = io::stdout().flush();
        self.inner.set_value(&buf);
        (self.validate)(&buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn focus_blur_done() {
        let mut t = Text::new();
        assert!(t.is_done());
        t.focus();
        assert!(!t.is_done());
        t.blur();
        assert!(t.is_done());
    }

    #[test]
    fn validate_passes_empty_by_default() {
        let t = Text::new();
        assert!(t.validate().is_ok());
    }

    #[test]
    fn validate_custom_fn() {
        let t = Text::new().with_validate(|s: &String| {
            if s.len() < 10 { Err("too short".into()) } else { Ok(()) }
        });
        assert!(t.validate().is_err());
    }

    #[test]
    fn view_contains_title() {
        let t = Text::new().with_title("Bio");
        assert!(t.view().contains("Bio"));
    }

    #[test]
    fn key_accessor() {
        let t = Text::new().with_key("bio");
        assert_eq!(t.key(), Some("bio"));
    }
}
