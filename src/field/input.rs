use bubbletea_rs::{Cmd, Msg};
use bubbles_rs::textinput;

use crate::keymap::InputKeyMap;
use crate::validate::ValidateFunc;
use super::Field;

/// Single-line text input field.
pub struct Input {
    title:       String,
    description: String,
    placeholder: String,
    inner:       textinput::Model,
    validate:    ValidateFunc<String>,
    err:         Option<String>,
    focused:     bool,
    key:         Option<String>,
    #[allow(dead_code)]
    keymap:      InputKeyMap,
}

impl Input {
    pub fn new() -> Self {
        Input {
            title:       String::new(),
            description: String::new(),
            placeholder: String::new(),
            inner:       textinput::Model::new(),
            validate:    crate::validate::no_op(),
            err:         None,
            focused:     false,
            key:         None,
            keymap:      InputKeyMap {
                accept_suggestion: bubbles_rs::key::Binding::new().with_keys(["ctrl+e"]),
                prev:  bubbles_rs::key::Binding::new().with_keys(["shift+tab"]),
                next:  bubbles_rs::key::Binding::new().with_keys(["enter", "tab"]),
                submit: bubbles_rs::key::Binding::new().with_keys(["enter"]),
            },
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

    pub fn with_placeholder(mut self, ph: impl Into<String>) -> Self {
        self.placeholder = ph.into();
        let ph = self.placeholder.clone();
        self.inner.set_value(&ph);
        self
    }

    pub fn with_key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    pub fn with_validate<F: Fn(&String) -> Result<(), String> + Send + 'static>(
        mut self,
        f: F,
    ) -> Self {
        self.validate = Box::new(f);
        self
    }

    /// Return the current value.
    pub fn value(&self) -> String {
        self.inner.value()
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}

impl Field for Input {
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
        let prompt = if self.title.is_empty() { "Input" } else { &self.title };
        print!("{}: ", prompt);
        io::stdout().flush().map_err(|e| e.to_string())?;
        let mut line = String::new();
        io::stdin().lock().read_line(&mut line).map_err(|e| e.to_string())?;
        let trimmed = line.trim_end_matches('\n').trim_end_matches('\r');
        self.inner.set_value(trimmed);
        (self.validate)(&trimmed.to_string()).map_err(|e| e)
    }

    fn value_string(&self) -> String {
        self.inner.value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn focus_blur_done() {
        let mut i = Input::new();
        assert!(i.is_done());
        i.focus();
        assert!(!i.is_done());
        i.blur();
        assert!(i.is_done());
    }

    #[test]
    fn validate_passes_empty_by_default() {
        let i = Input::new();
        assert!(i.validate().is_ok());
    }

    #[test]
    fn validate_custom_fn() {
        let i = Input::new()
            .with_validate(|s: &String| {
                if s.is_empty() { Err("required".into()) } else { Ok(()) }
            });
        assert!(i.validate().is_err());
    }

    #[test]
    fn view_contains_title() {
        let i = Input::new().with_title("Name");
        assert!(i.view().contains("Name"));
    }

    #[test]
    fn key_accessor() {
        let i = Input::new().with_key("name");
        assert_eq!(i.key(), Some("name"));
    }
}
