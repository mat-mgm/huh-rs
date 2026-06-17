pub mod confirm;
pub mod filepicker;
pub mod input;
pub mod multiselect;
pub mod note;
pub mod select;
pub mod text;

use bubbletea_rs::{Cmd, Msg};

/// Shared interface for all form field types.
pub trait Field: Send {
    /// Handle a message; return an optional command.
    fn update(&mut self, msg: &Msg) -> Option<Cmd>;

    /// Render the field to a string.
    fn view(&self) -> String;

    /// Give the field keyboard focus.
    fn focus(&mut self) -> Option<Cmd>;

    /// Remove keyboard focus.
    fn blur(&mut self);

    /// True when the field has a value and no pending error.
    fn is_done(&self) -> bool;

    /// Run field-level validation; return `Ok(())` or an error message.
    fn validate(&self) -> Result<(), String>;

    /// Optional unique key used to store the value in form results.
    fn key(&self) -> Option<&str> {
        None
    }

    /// Run the field in accessible (plain-text) mode, writing prompt to stdout
    /// and reading response from stdin.  Default: no-op (display-only fields).
    fn run_accessible(&mut self) -> Result<(), String> {
        Ok(())
    }

    /// Return the field's current value as a string. Used for value extraction
    /// after a form run. Default returns an empty string.
    fn value_string(&self) -> String {
        String::new()
    }
}
