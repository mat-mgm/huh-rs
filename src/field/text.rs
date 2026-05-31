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
}
