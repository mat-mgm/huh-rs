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
    pub fn run(self) -> Result<()> {
        use bubbletea_rs::Program;
        let program = run::FormProgram { form: self };
        // bubbletea_rs::Program::run() is async; block on it with tokio.
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(async {
            Program::new(program).run().await?;
            Ok(())
        })
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
