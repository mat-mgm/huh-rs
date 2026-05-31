use bubbletea_rs::{Cmd, Model, Msg, QuitMsg, View};

use crate::Form;

/// Wraps a `Form` as a `bubbletea_rs::Model` so it can be driven by the
/// runtime.  `Form::run()` constructs this and calls `Program::run()`.
pub(crate) struct FormProgram {
    pub form: Form,
}

impl Model for FormProgram {
    fn init(&mut self) -> Option<Cmd> {
        None
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Propagate to the form.
        let cmd = self.form.update(&msg);

        // Quit when the form is done or when a QuitMsg is received.
        if self.form.state() != crate::FormState::Normal || msg.is::<QuitMsg>() {
            return Some(bubbletea_rs::quit());
        }

        cmd
    }

    fn view(&self) -> View {
        View::new(self.form.view())
    }
}
