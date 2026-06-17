use bubbletea_rs::{Cmd, KeyPressMsg, Msg};
use bubbles_rs::key::matches_binding;

use crate::field::Field;
use crate::keymap::KeyMap;

/// A collection of fields displayed on a single "page" of the form.
pub struct Group {
    fields:      Vec<Box<dyn Field>>,
    title:       String,
    description: String,
    cursor:      usize,
    keymap:      KeyMap,
    width:       usize,
    height:      usize,
}

impl Group {
    pub fn new(fields: Vec<Box<dyn Field>>) -> Self {
        // Focus the first field immediately.
        let mut g = Group {
            fields,
            title:       String::new(),
            description: String::new(),
            cursor:      0,
            keymap:      KeyMap::default(),
            width:       80,
            height:      24,
        };
        if let Some(f) = g.fields.first_mut() {
            f.focus();
        }
        g
    }

    pub fn with_title(mut self, t: impl Into<String>) -> Self { self.title = t.into(); self }
    pub fn with_description(mut self, d: impl Into<String>) -> Self { self.description = d.into(); self }
    pub fn with_width(mut self, w: usize) -> Self { self.width = w; self }
    pub fn with_height(mut self, h: usize) -> Self { self.height = h; self }

    pub fn fields(&self) -> &[Box<dyn Field>] { &self.fields }

    /// True when every field in the group is done.
    pub fn is_done(&self) -> bool {
        self.fields.iter().all(|f| f.is_done())
    }

    /// True if all done fields pass validation.
    pub fn is_valid(&self) -> bool {
        self.fields.iter().all(|f| f.validate().is_ok())
    }

    fn advance_cursor(&mut self) {
        let n = self.fields.len();
        if self.cursor + 1 < n {
            self.fields[self.cursor].blur();
            self.cursor += 1;
            self.fields[self.cursor].focus();
        } else {
            // Last field: blur it to mark group done.
            self.fields[self.cursor].blur();
        }
    }

    fn retreat_cursor(&mut self) {
        if self.cursor > 0 {
            self.fields[self.cursor].blur();
            self.cursor -= 1;
            self.fields[self.cursor].focus();
        }
    }

    pub fn update(&mut self, msg: &Msg) -> Option<Cmd> {
        // Route quit / navigation keys at the group level.
        if let Some(kp) = msg.downcast_ref::<KeyPressMsg>() {
            let key = kp.to_string();
            if matches_binding(&key, &self.keymap.quit) {
                return Some(bubbletea_rs::quit());
            }
        }

        // Let the focused field handle the message first.
        let cmd = if let Some(f) = self.fields.get_mut(self.cursor) {
            f.update(msg)
        } else {
            None
        };

        // If the field just finished (blur triggered by enter/next), advance.
        if let Some(kp) = msg.downcast_ref::<KeyPressMsg>() {
            let key = kp.to_string();
            // Check whether the focused field consumed a "next" key.
            let next_pressed = key == "enter" || key == "tab";
            let prev_pressed = key == "shift+tab";
            if next_pressed && !self.is_done() {
                self.advance_cursor();
            } else if prev_pressed {
                self.retreat_cursor();
            }
        }

        cmd
    }

    pub fn run_accessible(&mut self) -> Result<(), String> {
        for field in &mut self.fields {
            field.run_accessible()?;
        }
        Ok(())
    }

    pub fn view(&self) -> String {
        let mut out = String::new();
        if !self.title.is_empty() { out.push_str(&self.title); out.push('\n'); }
        if !self.description.is_empty() { out.push_str(&self.description); out.push('\n'); }
        for f in &self.fields {
            out.push_str(&f.view());
        }
        out
    }
}
