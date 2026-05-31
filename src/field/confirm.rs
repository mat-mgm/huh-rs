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

    fn run_accessible(&mut self) -> Result<(), String> {
        use std::io::{self, BufRead, Write};
        let default_str = if self.value { "[Y/n]" } else { "[y/N]" };
        let prompt = if self.title.is_empty() { "Choose" } else { &self.title };
        print!("{} {}: ", prompt, default_str);
        io::stdout().flush().map_err(|e| e.to_string())?;
        let mut line = String::new();
        io::stdin().lock().read_line(&mut line).map_err(|e| e.to_string())?;
        let trimmed = line.trim().to_lowercase();
        self.value = match trimmed.as_str() {
            "y" | "yes" => true,
            "n" | "no"  => false,
            ""          => self.value,
            _           => return Err(format!("invalid input: {}", trimmed)),
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bubbletea_rs::{Key, KeyCode, KeyMod, KeyPressMsg};

    fn key_msg(code: KeyCode) -> bubbletea_rs::Msg {
        bubbletea_rs::message::msg(KeyPressMsg(Key {
            code,
            modifiers: KeyMod::default(),
            text: match code {
                KeyCode::Char(c) => c.to_string(),
                _ => String::new(),
            },
            is_repeat: false,
        }))
    }

    #[test]
    fn toggle_with_h_l() {
        let mut c = Confirm::new().with_value(false);
        c.focus();
        c.update(&key_msg(KeyCode::Char('h')));
        assert!(c.value());
        c.update(&key_msg(KeyCode::Char('l')));
        assert!(!c.value());
    }

    #[test]
    fn accept_reject_keys() {
        let mut c = Confirm::new().with_value(false);
        c.focus();
        c.update(&key_msg(KeyCode::Char('y')));
        assert!(c.value());
        c.update(&key_msg(KeyCode::Char('n')));
        assert!(!c.value());
    }

    #[test]
    fn no_update_when_not_focused() {
        let mut c = Confirm::new().with_value(false);
        c.update(&key_msg(KeyCode::Char('y')));
        assert!(!c.value());
    }

    #[test]
    fn view_shows_selection() {
        let c = Confirm::new().with_value(true);
        let v = c.view();
        assert!(v.contains("[ Yes ]"));
        let c = Confirm::new().with_value(false);
        let v = c.view();
        assert!(v.contains("[ No ]"));
    }

    #[test]
    fn validate_always_ok() {
        let c = Confirm::new();
        assert!(c.validate().is_ok());
    }
}
