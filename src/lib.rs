pub mod field;
pub mod group;
pub mod keymap;
pub mod layout;
pub mod run;
pub mod theme;
pub mod validate;

pub use field::Field;
pub use field::select::FieldOption;
pub use group::Group;
pub use keymap::KeyMap;
pub use theme::{BaseTheme, CatppuccinTheme, CharmTheme, FieldStyles, Styles, Theme};
pub use validate::ValidateFunc;

use bubbletea_rs::{Cmd, Msg};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

/// Current state of the form.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormState {
    Normal,
    Completed,
    Aborted,
}

/// A multi-step form composed of one or more groups.
pub struct Form {
    groups:     Vec<Group>,
    cursor:     usize,
    state:      FormState,
    accessible: bool,
    width:      usize,
    height:     usize,
}

impl Form {
    pub fn new(groups: Vec<Group>) -> Self {
        Form {
            groups,
            cursor:     0,
            state:      FormState::Normal,
            accessible: false,
            width:      80,
            height:     24,
        }
    }

    pub fn with_width(mut self, w: usize) -> Self { self.width = w; self }
    pub fn with_height(mut self, h: usize) -> Self { self.height = h; self }
    pub fn with_accessible(mut self, v: bool) -> Self { self.accessible = v; self }

    pub fn state(&self) -> FormState { self.state }

    pub fn groups(&self) -> &[Group] { &self.groups }

    pub fn update(&mut self, msg: &Msg) -> Option<Cmd> {
        use bubbletea_rs::{InterruptMsg, QuitMsg};

        if msg.is::<QuitMsg>() || msg.is::<InterruptMsg>() {
            self.state = FormState::Aborted;
            return Some(bubbletea_rs::quit());
        }

        let cmd = if let Some(g) = self.groups.get_mut(self.cursor) {
            g.update(msg)
        } else {
            None
        };

        // Advance to next group when current group is done.
        if let Some(g) = self.groups.get(self.cursor) {
            if g.is_done() && g.is_valid() {
                self.cursor += 1;
                if self.cursor >= self.groups.len() {
                    self.state = FormState::Completed;
                    return Some(bubbletea_rs::quit());
                }
            }
        }

        cmd
    }

    pub fn view(&self) -> String {
        if let Some(g) = self.groups.get(self.cursor) {
            g.view()
        } else {
            String::new()
        }
    }

    /// Run the form as a standalone blocking program.
    /// Returns `Ok(())` on completion, `Err(ErrUserAborted)` on abort.
    pub fn run(mut self) -> Result<()> {
        if self.accessible {
            return self.run_accessible();
        }
        use bubbletea_rs::Program;
        let program = run::FormProgram { form: self };
        // bubbletea_rs::Program::run() is async; block on it with tokio.
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(async {
            Program::new(program).run().await?;
            Ok(())
        })
    }

    /// Run the form in accessible (plain-text stdin/stdout) mode.
    /// Values are retained in the form fields after this returns; access
    /// them via `groups()[i].fields()[j].value_string()`.
    pub fn run_accessible(&mut self) -> Result<()> {
        for group in &mut self.groups {
            group.run_accessible().map_err(|e| -> Error { e.into() })?;
        }
        Ok(())
    }
}

// Convenience constructors ---------------------------------------------------

/// Create a `Group` from a list of boxed fields.
pub fn group(fields: Vec<Box<dyn Field>>) -> Group {
    Group::new(fields)
}

/// Wrap a field in a box for use in a group.
#[macro_export]
macro_rules! field {
    ($f:expr) => { Box::new($f) as Box<dyn $crate::Field> };
}

#[cfg(test)]
mod tests {
    use super::*;
    use bubbletea_rs::{Key, KeyCode, KeyMod, KeyPressMsg};

    fn key_msg(code: KeyCode) -> Msg {
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

    // Group tests

    #[test]
    fn group_not_done_initially() {
        use field::confirm::Confirm;
        let c = Confirm::new();
        let g = Group::new(vec![field!(c)]);
        // First field starts focused, so group is not done.
        assert!(!g.is_done());
    }

    #[test]
    fn group_is_done_after_single_field_enters() {
        use field::confirm::Confirm;
        let c = Confirm::new();
        let mut g = Group::new(vec![field!(c)]);
        assert!(!g.is_done());
        g.update(&key_msg(KeyCode::Enter));
        assert!(g.is_done());
    }

    #[test]
    fn group_two_fields_advance_then_retreat() {
        use field::confirm::Confirm;
        let c1 = Confirm::new();
        let c2 = Confirm::new();
        let mut g = Group::new(vec![field!(c1), field!(c2)]);
        // After enter, first field blurs, second gets focus → group not done yet.
        g.update(&key_msg(KeyCode::Enter));
        assert!(!g.is_done());
        // Shift+Tab retreats back to first field.
        g.update(&key_msg(KeyCode::BackTab));
        // Both fields focused again scenario: group still not done.
        assert!(!g.is_done());
    }

    // Form tests

    #[test]
    fn form_initial_state_is_normal() {
        let f = Form::new(vec![]);
        assert_eq!(f.state(), FormState::Normal);
    }

    #[test]
    fn form_completes_when_all_groups_done() {
        use field::confirm::Confirm;
        let c = Confirm::new();
        let mut f = Form::new(vec![Group::new(vec![field!(c)])]);
        // Enter on the single confirm field should complete the form.
        f.update(&key_msg(KeyCode::Enter));
        assert_eq!(f.state(), FormState::Completed);
    }

    #[test]
    fn form_layout_column_width() {
        use layout::Layout;
        assert_eq!(Layout::Columns(2).column_width(80), 40);
        assert_eq!(Layout::Stack.column_width(80), 80);
    }
}
