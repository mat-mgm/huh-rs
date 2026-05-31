use bubbletea_rs::{Cmd, KeyPressMsg, Msg};
use bubbles_rs::key::matches_binding;

use crate::keymap::ConfirmKeyMap;
use super::Field;

/// Yes/No confirmation field.
pub struct Confirm {
    title:            String,
    description:      String,
    value:            bool,
    affirmative:      String,
    negative:         String,
    focused:          bool,
    key:              Option<String>,
    keymap:           ConfirmKeyMap,
}

impl Confirm {
    pub fn new() -> Self {
        Confirm {
            title:       String::new(),
            description: String::new(),
            value:       false,
            affirmative: "Yes".to_string(),
            negative:    "No".to_string(),
            focused:     false,
            key:         None,
            keymap: ConfirmKeyMap {
                prev:   bubbles_rs::key::Binding::new().with_keys(["shift+tab"]),
                next:   bubbles_rs::key::Binding::new().with_keys(["enter", "tab"]),
                toggle: bubbles_rs::key::Binding::new().with_keys(["h", "l", "right", "left"]),
                submit: bubbles_rs::key::Binding::new().with_keys(["enter"]),
                accept: bubbles_rs::key::Binding::new().with_keys(["y", "Y"]),
                reject: bubbles_rs::key::Binding::new().with_keys(["n", "N"]),
            },
        }
    }

    pub fn with_title(mut self, t: impl Into<String>) -> Self { self.title = t.into(); self }
    pub fn with_description(mut self, d: impl Into<String>) -> Self { self.description = d.into(); self }
    pub fn with_value(mut self, v: bool) -> Self { self.value = v; self }
    pub fn with_affirmative(mut self, s: impl Into<String>) -> Self { self.affirmative = s.into(); self }
    pub fn with_negative(mut self, s: impl Into<String>) -> Self { self.negative = s.into(); self }
    pub fn with_key(mut self, k: impl Into<String>) -> Self { self.key = Some(k.into()); self }

    pub fn value(&self) -> bool { self.value }
}

impl Default for Confirm {
    fn default() -> Self { Self::new() }
}

impl Field for Confirm {
    fn update(&mut self, msg: &Msg) -> Option<Cmd> {
        if !self.focused { return None; }
        if let Some(kp) = msg.downcast_ref::<KeyPressMsg>() {
            let key = kp.to_string();
            if matches_binding(&key, &self.keymap.toggle) {
                self.value = !self.value;
            } else if matches_binding(&key, &self.keymap.accept) {
                self.value = true;
            } else if matches_binding(&key, &self.keymap.reject) {
                self.value = false;
            }
        }
        None
    }

    fn view(&self) -> String {
        let mut out = String::new();
        if !self.title.is_empty() { out.push_str(&self.title); out.push('\n'); }
        if !self.description.is_empty() { out.push_str(&self.description); out.push('\n'); }
        let (yes_mark, no_mark) = if self.value {
            ("[ Yes ]", "  No  ")
        } else {
            ("  Yes  ", "[ No ]")
        };
        out.push_str(&format!("{yes_mark}  {no_mark}\n"));
        out
    }

    fn focus(&mut self) -> Option<Cmd> { self.focused = true; None }
    fn blur(&mut self) { self.focused = false; }
    fn is_done(&self) -> bool { !self.focused }
    fn validate(&self) -> Result<(), String> { Ok(()) }
    fn key(&self) -> Option<&str> { self.key.as_deref() }
}
