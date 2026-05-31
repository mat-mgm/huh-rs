use bubbles_rs::key::Binding;

/// Keybindings for the quit action (global).
#[derive(Clone, Debug)]
pub struct KeyMap {
    pub quit:         Binding,
    pub input:        InputKeyMap,
    pub text:         TextKeyMap,
    pub select:       SelectKeyMap,
    pub multi_select: MultiSelectKeyMap,
    pub confirm:      ConfirmKeyMap,
    pub note:         NoteKeyMap,
    pub file_picker:  FilePickerKeyMap,
}

#[derive(Clone, Debug)]
pub struct InputKeyMap {
    pub accept_suggestion: Binding,
    pub next:              Binding,
    pub prev:              Binding,
    pub submit:            Binding,
}

#[derive(Clone, Debug)]
pub struct TextKeyMap {
    pub next:     Binding,
    pub prev:     Binding,
    pub new_line: Binding,
    pub submit:   Binding,
}

#[derive(Clone, Debug)]
pub struct SelectKeyMap {
    pub next:          Binding,
    pub prev:          Binding,
    pub up:            Binding,
    pub down:          Binding,
    pub half_page_up:  Binding,
    pub half_page_down: Binding,
    pub goto_top:      Binding,
    pub goto_bottom:   Binding,
    pub filter:        Binding,
    pub set_filter:    Binding,
    pub clear_filter:  Binding,
    pub submit:        Binding,
}

#[derive(Clone, Debug)]
pub struct MultiSelectKeyMap {
    pub next:          Binding,
    pub prev:          Binding,
    pub up:            Binding,
    pub down:          Binding,
    pub half_page_up:  Binding,
    pub half_page_down: Binding,
    pub goto_top:      Binding,
    pub goto_bottom:   Binding,
    pub toggle:        Binding,
    pub filter:        Binding,
    pub set_filter:    Binding,
    pub clear_filter:  Binding,
    pub submit:        Binding,
    pub select_all:    Binding,
    pub select_none:   Binding,
}

#[derive(Clone, Debug)]
pub struct ConfirmKeyMap {
    pub next:   Binding,
    pub prev:   Binding,
    pub toggle: Binding,
    pub submit: Binding,
    pub accept: Binding,
    pub reject: Binding,
}

#[derive(Clone, Debug)]
pub struct NoteKeyMap {
    pub next:   Binding,
    pub prev:   Binding,
    pub submit: Binding,
}

#[derive(Clone, Debug)]
pub struct FilePickerKeyMap {
    pub open:        Binding,
    pub close:       Binding,
    pub goto_top:    Binding,
    pub goto_bottom: Binding,
    pub page_up:     Binding,
    pub page_down:   Binding,
    pub back:        Binding,
    pub select:      Binding,
    pub up:          Binding,
    pub down:        Binding,
    pub prev:        Binding,
    pub next:        Binding,
    pub submit:      Binding,
}

impl Default for KeyMap {
    fn default() -> Self {
        Self {
            quit: Binding::new().with_keys(["ctrl+c"]),
            input: InputKeyMap {
                accept_suggestion: Binding::new().with_keys(["ctrl+e"]),
                prev:  Binding::new().with_keys(["shift+tab"]),
                next:  Binding::new().with_keys(["enter", "tab"]),
                submit: Binding::new().with_keys(["enter"]),
            },
            text: TextKeyMap {
                prev:     Binding::new().with_keys(["shift+tab"]),
                next:     Binding::new().with_keys(["tab"]),
                new_line: Binding::new().with_keys(["alt+enter", "ctrl+j"]),
                submit:   Binding::new().with_keys(["enter"]),
            },
            select: SelectKeyMap {
                prev:           Binding::new().with_keys(["shift+tab"]),
                next:           Binding::new().with_keys(["enter", "tab"]),
                up:             Binding::new().with_keys(["up", "k", "ctrl+k", "ctrl+p"]),
                down:           Binding::new().with_keys(["down", "j", "ctrl+j", "ctrl+n"]),
                half_page_up:   Binding::new().with_keys(["ctrl+u"]),
                half_page_down: Binding::new().with_keys(["ctrl+d"]),
                goto_top:       Binding::new().with_keys(["home", "g"]),
                goto_bottom:    Binding::new().with_keys(["end", "G"]),
                filter:         Binding::new().with_keys(["/"]),
                set_filter:     Binding::new().with_keys(["esc"]),
                clear_filter:   Binding::new().with_keys(["esc"]),
                submit:         Binding::new().with_keys(["enter"]),
            },
            multi_select: MultiSelectKeyMap {
                prev:           Binding::new().with_keys(["shift+tab"]),
                next:           Binding::new().with_keys(["enter", "tab"]),
                up:             Binding::new().with_keys(["up", "k", "ctrl+p"]),
                down:           Binding::new().with_keys(["down", "j", "ctrl+n"]),
                half_page_up:   Binding::new().with_keys(["ctrl+u"]),
                half_page_down: Binding::new().with_keys(["ctrl+d"]),
                goto_top:       Binding::new().with_keys(["home", "g"]),
                goto_bottom:    Binding::new().with_keys(["end", "G"]),
                toggle:         Binding::new().with_keys(["space", "x"]),
                filter:         Binding::new().with_keys(["/"]),
                set_filter:     Binding::new().with_keys(["enter", "esc"]),
                clear_filter:   Binding::new().with_keys(["esc"]),
                submit:         Binding::new().with_keys(["enter"]),
                select_all:     Binding::new().with_keys(["ctrl+a"]),
                select_none:    Binding::new().with_keys(["ctrl+a"]),
            },
            confirm: ConfirmKeyMap {
                prev:   Binding::new().with_keys(["shift+tab"]),
                next:   Binding::new().with_keys(["enter", "tab"]),
                toggle: Binding::new().with_keys(["h", "l", "right", "left"]),
                submit: Binding::new().with_keys(["enter"]),
                accept: Binding::new().with_keys(["y", "Y"]),
                reject: Binding::new().with_keys(["n", "N"]),
            },
            note: NoteKeyMap {
                prev:   Binding::new().with_keys(["shift+tab"]),
                next:   Binding::new().with_keys(["enter", "tab"]),
                submit: Binding::new().with_keys(["enter"]),
            },
            file_picker: FilePickerKeyMap {
                open:        Binding::new().with_keys(["l", "right", "enter"]),
                close:       Binding::new().with_keys(["esc"]),
                goto_top:    Binding::new().with_keys(["g"]),
                goto_bottom: Binding::new().with_keys(["G"]),
                page_up:     Binding::new().with_keys(["K", "pgup"]),
                page_down:   Binding::new().with_keys(["J", "pgdown"]),
                back:        Binding::new().with_keys(["h", "backspace", "left", "esc"]),
                select:      Binding::new().with_keys(["enter"]),
                up:          Binding::new().with_keys(["up", "k", "ctrl+k", "ctrl+p"]),
                down:        Binding::new().with_keys(["down", "j", "ctrl+j", "ctrl+n"]),
                prev:        Binding::new().with_keys(["shift+tab"]),
                next:        Binding::new().with_keys(["tab"]),
                submit:      Binding::new().with_keys(["enter"]),
            },
        }
    }
}
