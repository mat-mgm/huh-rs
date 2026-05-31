use bubbletea_rs::{Cmd, KeyPressMsg, Msg};
use bubbles_rs::key::matches_binding;

use crate::keymap::SelectKeyMap;
use crate::validate::ValidateFunc;
use super::Field;

/// A labelled option value.
#[derive(Clone)]
pub struct FieldOption<T: Clone> {
    pub label: String,
    pub value: T,
}

impl<T: Clone> FieldOption<T> {
    pub fn new(label: impl Into<String>, value: T) -> Self {
        FieldOption { label: label.into(), value }
    }
}

/// Single-choice select field.
pub struct Select<T: Clone + Send> {
    title:       String,
    description: String,
    options:     Vec<FieldOption<T>>,
    cursor:      usize,
    focused:     bool,
    key:         Option<String>,
    validate:    ValidateFunc<T>,
    err:         Option<String>,
    keymap:      SelectKeyMap,
    filter_text: String,
    filtering:   bool,
}

impl<T: Clone + Send> Select<T> {
    pub fn new() -> Self {
        Select {
            title:       String::new(),
            description: String::new(),
            options:     Vec::new(),
            cursor:      0,
            focused:     false,
            key:         None,
            validate:    crate::validate::no_op(),
            err:         None,
            keymap: SelectKeyMap {
                prev:           bubbles_rs::key::Binding::new().with_keys(["shift+tab"]),
                next:           bubbles_rs::key::Binding::new().with_keys(["enter", "tab"]),
                up:             bubbles_rs::key::Binding::new().with_keys(["up", "k", "ctrl+k", "ctrl+p"]),
                down:           bubbles_rs::key::Binding::new().with_keys(["down", "j", "ctrl+j", "ctrl+n"]),
                half_page_up:   bubbles_rs::key::Binding::new().with_keys(["ctrl+u"]),
                half_page_down: bubbles_rs::key::Binding::new().with_keys(["ctrl+d"]),
                goto_top:       bubbles_rs::key::Binding::new().with_keys(["home", "g"]),
                goto_bottom:    bubbles_rs::key::Binding::new().with_keys(["end", "G"]),
                filter:         bubbles_rs::key::Binding::new().with_keys(["/"]),
                set_filter:     bubbles_rs::key::Binding::new().with_keys(["esc"]),
                clear_filter:   bubbles_rs::key::Binding::new().with_keys(["esc"]),
                submit:         bubbles_rs::key::Binding::new().with_keys(["enter"]),
            },
            filter_text: String::new(),
            filtering:   false,
        }
    }

    pub fn with_title(mut self, t: impl Into<String>) -> Self {
        self.title = t.into(); self
    }

    pub fn with_description(mut self, d: impl Into<String>) -> Self {
        self.description = d.into(); self
    }

    pub fn with_options(mut self, opts: Vec<FieldOption<T>>) -> Self {
        self.options = opts; self
    }

    pub fn with_key(mut self, k: impl Into<String>) -> Self {
        self.key = Some(k.into()); self
    }

    pub fn with_validate<F: Fn(&T) -> Result<(), String> + Send + 'static>(
        mut self, f: F,
    ) -> Self {
        self.validate = Box::new(f); self
    }

    /// Filtered options based on current filter text.
    fn visible(&self) -> Vec<(usize, &FieldOption<T>)> {
        if self.filter_text.is_empty() {
            self.options.iter().enumerate().collect()
        } else {
            let ft = self.filter_text.to_lowercase();
            self.options.iter().enumerate()
                .filter(|(_, o)| o.label.to_lowercase().contains(&ft))
                .collect()
        }
    }

    /// Current selected value (if any).
    pub fn value(&self) -> Option<&T> {
        self.options.get(self.cursor).map(|o| &o.value)
    }
}

impl<T: Clone + Send> Default for Select<T> {
    fn default() -> Self { Self::new() }
}

impl<T: Clone + Send + 'static> Field for Select<T> {
    fn update(&mut self, msg: &Msg) -> Option<Cmd> {
        if !self.focused { return None; }
        if let Some(kp) = msg.downcast_ref::<KeyPressMsg>() {
            let key = kp.to_string();
            if self.filtering {
                if matches_binding(&key, &self.keymap.set_filter)
                    || matches_binding(&key, &self.keymap.clear_filter) {
                    self.filtering = false;
                } else if key == "backspace" {
                    self.filter_text.pop();
                } else if key.len() == 1 {
                    self.filter_text.push_str(&key);
                }
                return None;
            }
            let vis = self.visible();
            let vis_len = vis.len();
            if vis_len == 0 { return None; }
            let cur_vis = vis.iter().position(|(i, _)| *i == self.cursor).unwrap_or(0);

            if matches_binding(&key, &self.keymap.up) {
                let new = if cur_vis == 0 { vis_len - 1 } else { cur_vis - 1 };
                self.cursor = vis[new].0;
            } else if matches_binding(&key, &self.keymap.down) {
                let new = (cur_vis + 1) % vis_len;
                self.cursor = vis[new].0;
            } else if matches_binding(&key, &self.keymap.goto_top) {
                self.cursor = vis[0].0;
            } else if matches_binding(&key, &self.keymap.goto_bottom) {
                self.cursor = vis[vis_len - 1].0;
            } else if matches_binding(&key, &self.keymap.filter) {
                self.filtering = true;
                self.filter_text.clear();
            }
        }
        None
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
        if self.filtering {
            out.push_str(&format!("> {}\n", self.filter_text));
        }
        for (orig_idx, opt) in self.visible() {
            let sel = orig_idx == self.cursor;
            let prefix = if sel { "> " } else { "  " };
            out.push_str(&format!("{}{}\n", prefix, opt.label));
        }
        if let Some(e) = &self.err {
            out.push_str(e);
            out.push('\n');
        }
        out
    }

    fn focus(&mut self) -> Option<Cmd> {
        self.focused = true;
        None
    }

    fn blur(&mut self) {
        self.focused = false;
    }

    fn is_done(&self) -> bool {
        !self.focused
    }

    fn validate(&self) -> Result<(), String> {
        match self.options.get(self.cursor) {
            None => Err("no option selected".to_string()),
            Some(o) => (self.validate)(&o.value),
        }
    }

    fn key(&self) -> Option<&str> {
        self.key.as_deref()
    }

    fn run_accessible(&mut self) -> Result<(), String> {
        use std::io::{self, BufRead, Write};
        let prompt = if self.title.is_empty() { "Select" } else { &self.title };
        println!("{}:", prompt);
        for (i, opt) in self.options.iter().enumerate() {
            println!("  {}: {}", i + 1, opt.label);
        }
        let n = self.options.len();
        loop {
            print!("Enter number (1-{}): ", n);
            io::stdout().flush().map_err(|e| e.to_string())?;
            let mut line = String::new();
            io::stdin().lock().read_line(&mut line).map_err(|e| e.to_string())?;
            let trimmed = line.trim();
            if let Ok(idx) = trimmed.parse::<usize>() {
                if idx >= 1 && idx <= n {
                    self.cursor = idx - 1;
                    return (self.validate)(&self.options[self.cursor].value);
                }
            }
            eprintln!("Please enter a number between 1 and {}.", n);
        }
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

    fn opts() -> Vec<FieldOption<&'static str>> {
        vec![
            FieldOption::new("Alpha", "a"),
            FieldOption::new("Beta", "b"),
            FieldOption::new("Gamma", "c"),
        ]
    }

    #[test]
    fn navigate_down_and_up() {
        let mut s = Select::new().with_options(opts());
        s.focus();
        assert_eq!(s.value(), Some(&"a"));
        s.update(&key_msg(KeyCode::Down));
        assert_eq!(s.value(), Some(&"b"));
        s.update(&key_msg(KeyCode::Up));
        assert_eq!(s.value(), Some(&"a"));
    }

    #[test]
    fn goto_top_bottom() {
        let mut s = Select::new().with_options(opts());
        s.focus();
        s.update(&key_msg(KeyCode::End));
        assert_eq!(s.value(), Some(&"c"));
        s.update(&key_msg(KeyCode::Home));
        assert_eq!(s.value(), Some(&"a"));
    }

    #[test]
    fn down_wraps_around() {
        let mut s = Select::new().with_options(opts());
        s.focus();
        s.update(&key_msg(KeyCode::End));
        s.update(&key_msg(KeyCode::Down));
        assert_eq!(s.value(), Some(&"a"));
    }

    #[test]
    fn filter_narrows_options() {
        let mut s = Select::new().with_options(opts());
        s.focus();
        // Enter filter mode with '/'
        s.update(&key_msg(KeyCode::Char('/')));
        assert!(s.filtering);
        // Type 'b' to filter
        s.update(&key_msg(KeyCode::Char('b')));
        assert_eq!(s.filter_text, "b");
        // Only Beta visible; exit filter with esc
        s.update(&key_msg(KeyCode::Esc));
        assert!(!s.filtering);
    }

    #[test]
    fn view_marks_selection() {
        let s = Select::new().with_options(opts());
        let v = s.view();
        assert!(v.contains("> Alpha") || v.contains("> "));
    }

    #[test]
    fn validate_no_options_is_err() {
        let s = Select::<&str>::new();
        assert!(s.validate().is_err());
    }

    #[test]
    fn validate_with_option_is_ok() {
        let s = Select::new().with_options(opts());
        assert!(s.validate().is_ok());
    }
}
