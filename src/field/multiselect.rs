use std::collections::HashSet;

use bubbletea_rs::{Cmd, KeyPressMsg, Msg};
use bubbles_rs::key::matches_binding;

use crate::keymap::MultiSelectKeyMap;
use crate::validate::ValidateFunc;
use super::{Field, select::FieldOption};

/// Multi-choice select field.
pub struct MultiSelect<T: Clone + Send + Eq + std::hash::Hash> {
    title:       String,
    description: String,
    options:     Vec<FieldOption<T>>,
    selected:    HashSet<usize>,
    cursor:      usize,
    limit:       Option<usize>,
    focused:     bool,
    key:         Option<String>,
    validate:    ValidateFunc<Vec<T>>,
    err:         Option<String>,
    keymap:      MultiSelectKeyMap,
    filter_text: String,
    filtering:   bool,
}

impl<T: Clone + Send + Eq + std::hash::Hash> MultiSelect<T> {
    pub fn new() -> Self {
        MultiSelect {
            title:       String::new(),
            description: String::new(),
            options:     Vec::new(),
            selected:    HashSet::new(),
            cursor:      0,
            limit:       None,
            focused:     false,
            key:         None,
            validate:    crate::validate::no_op(),
            err:         None,
            keymap: MultiSelectKeyMap {
                prev:           bubbles_rs::key::Binding::new().with_keys(["shift+tab"]),
                next:           bubbles_rs::key::Binding::new().with_keys(["enter", "tab"]),
                up:             bubbles_rs::key::Binding::new().with_keys(["up", "k", "ctrl+p"]),
                down:           bubbles_rs::key::Binding::new().with_keys(["down", "j", "ctrl+n"]),
                half_page_up:   bubbles_rs::key::Binding::new().with_keys(["ctrl+u"]),
                half_page_down: bubbles_rs::key::Binding::new().with_keys(["ctrl+d"]),
                goto_top:       bubbles_rs::key::Binding::new().with_keys(["home", "g"]),
                goto_bottom:    bubbles_rs::key::Binding::new().with_keys(["end", "G"]),
                toggle:         bubbles_rs::key::Binding::new().with_keys(["space", "x"]),
                filter:         bubbles_rs::key::Binding::new().with_keys(["/"]),
                set_filter:     bubbles_rs::key::Binding::new().with_keys(["enter", "esc"]),
                clear_filter:   bubbles_rs::key::Binding::new().with_keys(["esc"]),
                submit:         bubbles_rs::key::Binding::new().with_keys(["enter"]),
                select_all:     bubbles_rs::key::Binding::new().with_keys(["ctrl+a"]),
                select_none:    bubbles_rs::key::Binding::new().with_keys(["ctrl+a"]),
            },
            filter_text: String::new(),
            filtering:   false,
        }
    }

    pub fn with_title(mut self, t: impl Into<String>) -> Self { self.title = t.into(); self }
    pub fn with_description(mut self, d: impl Into<String>) -> Self { self.description = d.into(); self }
    pub fn with_options(mut self, opts: Vec<FieldOption<T>>) -> Self { self.options = opts; self }
    pub fn with_limit(mut self, n: usize) -> Self { self.limit = Some(n); self }
    pub fn with_key(mut self, k: impl Into<String>) -> Self { self.key = Some(k.into()); self }

    pub fn with_validate<F: Fn(&Vec<T>) -> Result<(), String> + Send + 'static>(
        mut self, f: F,
    ) -> Self {
        self.validate = Box::new(f); self
    }

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

    pub fn values(&self) -> Vec<&T> {
        let mut out: Vec<_> = self.selected.iter()
            .filter_map(|i| self.options.get(*i).map(|o| &o.value))
            .collect();
        out.sort_by_key(|_| 0usize); // stable order
        out
    }
}

impl<T: Clone + Send + Eq + std::hash::Hash> Default for MultiSelect<T> {
    fn default() -> Self { Self::new() }
}

impl<T: Clone + Send + Eq + std::hash::Hash + 'static> Field for MultiSelect<T> {
    fn update(&mut self, msg: &Msg) -> Option<Cmd> {
        if !self.focused { return None; }
        if let Some(kp) = msg.downcast_ref::<KeyPressMsg>() {
            let key = kp.to_string();
            if self.filtering {
                if matches_binding(&key, &self.keymap.set_filter) {
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
            } else if matches_binding(&key, &self.keymap.toggle) {
                if self.selected.contains(&self.cursor) {
                    self.selected.remove(&self.cursor);
                } else {
                    let at_limit = self.limit.map(|lim| self.selected.len() >= lim).unwrap_or(false);
                    if !at_limit {
                        self.selected.insert(self.cursor);
                    }
                }
            } else if matches_binding(&key, &self.keymap.select_all) {
                for i in 0..self.options.len() { self.selected.insert(i); }
            } else if matches_binding(&key, &self.keymap.filter) {
                self.filtering = true;
                self.filter_text.clear();
            }
        }
        None
    }

    fn view(&self) -> String {
        let mut out = String::new();
        if !self.title.is_empty() { out.push_str(&self.title); out.push('\n'); }
        if !self.description.is_empty() { out.push_str(&self.description); out.push('\n'); }
        if self.filtering { out.push_str(&format!("> {}\n", self.filter_text)); }
        for (orig_idx, opt) in self.visible() {
            let is_cursor = orig_idx == self.cursor;
            let is_sel    = self.selected.contains(&orig_idx);
            let sel_mark  = if is_sel { "[x]" } else { "[ ]" };
            let cur_mark  = if is_cursor { ">" } else { " " };
            out.push_str(&format!("{} {} {}\n", cur_mark, sel_mark, opt.label));
        }
        if let Some(e) = &self.err { out.push_str(e); out.push('\n'); }
        out
    }

    fn focus(&mut self) -> Option<Cmd> { self.focused = true; None }
    fn blur(&mut self)  { self.focused = false; }
    fn is_done(&self) -> bool { !self.focused }

    fn validate(&self) -> Result<(), String> {
        let vals: Vec<T> = self.selected.iter()
            .filter_map(|i| self.options.get(*i).map(|o| o.value.clone()))
            .collect();
        (self.validate)(&vals)
    }

    fn key(&self) -> Option<&str> { self.key.as_deref() }
}
