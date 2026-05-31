use lipgloss_rs::{Color, Style};

/// Styles for a single field's focused or blurred state.
#[derive(Clone, Default)]
pub struct FieldStyles {
    pub base:            Style,
    pub title:           Style,
    pub description:     Style,
    pub error_indicator: Style,
    pub error_message:   Style,

    // Select / MultiSelect.
    pub select_selector:      Style,
    pub option:               Style,
    pub next_indicator:       Style,
    pub prev_indicator:       Style,
    pub multi_select_selector: Style,
    pub selected_option:      Style,
    pub selected_prefix:      Style,
    pub unselected_option:    Style,
    pub unselected_prefix:    Style,

    // FilePicker.
    pub directory: Style,
    pub file:      Style,

    // TextInput / Textarea.
    pub text_input: TextInputStyles,

    // Confirm.
    pub focused_button: Style,
    pub blurred_button: Style,

    // Note / card.
    pub card:       Style,
    pub note_title: Style,
    pub next:       Style,
}

/// Cursor, placeholder, and prompt styles for text inputs.
#[derive(Clone, Default)]
pub struct TextInputStyles {
    pub cursor:      Style,
    pub cursor_text: Style,
    pub placeholder: Style,
    pub prompt:      Style,
    pub text:        Style,
}

/// Top-level collection of styles for an entire form.
#[derive(Clone, Default)]
pub struct Styles {
    pub form:            FormStyles,
    pub group:           GroupStyles,
    pub field_separator: Style,
    pub blurred:         FieldStyles,
    pub focused:         FieldStyles,
}

#[derive(Clone, Default)]
pub struct FormStyles {
    pub base: Style,
}

#[derive(Clone, Default)]
pub struct GroupStyles {
    pub base:        Style,
    pub title:       Style,
    pub description: Style,
}

/// A theme produces a `Styles` for a given terminal background.
pub trait Theme: Send + Sync {
    fn styles(&self, is_dark: bool) -> Styles;
}

// ─── Built-in themes ─────────────────────────────────────────────────────────

/// Minimal base theme: no colors, just layout.
pub struct BaseTheme;
impl Theme for BaseTheme {
    fn styles(&self, _is_dark: bool) -> Styles {
        let mut s = Styles::default();
        s.focused.title = Style::new().bold(true);
        s.focused.focused_button = Style::new()
            .bold(true)
            .foreground(Color::parse("7"))
            .background(Color::parse("8"))
            .padding(&[0, 3]);
        s.focused.blurred_button = Style::new()
            .foreground(Color::parse("8"))
            .padding(&[0, 3]);
        s.blurred.focused_button = s.focused.blurred_button.clone();
        s.blurred.blurred_button = s.focused.blurred_button.clone();
        s
    }
}

/// Charm default theme.
pub struct CharmTheme;
impl Theme for CharmTheme {
    fn styles(&self, is_dark: bool) -> Styles {
        let mut s = BaseTheme.styles(is_dark);

        let pink    = Color::parse("#FF5D8F");
        let yellow  = Color::parse("#FFFFD2");
        let green   = Color::parse("#A1EFD3");
        let purple  = Color::parse("#7D56F4");
        let dark_fg = Color::parse("#585858");

        s.focused.title = Style::new().foreground(pink).bold(true);
        s.focused.description = Style::new().foreground(dark_fg);
        s.focused.base = Style::new()
            .border(lipgloss_rs::border::rounded_border(), &[])
            .border_foreground(&[pink]);
        s.focused.select_selector = Style::new().foreground(pink);
        s.focused.multi_select_selector = Style::new().foreground(pink);
        s.focused.selected_option = Style::new().foreground(green);
        s.focused.selected_prefix = Style::new().foreground(green);
        s.focused.unselected_option = Style::new().foreground(dark_fg);
        s.focused.focused_button = Style::new()
            .bold(true)
            .foreground(yellow)
            .background(purple)
            .padding(&[0, 3]);
        s.focused.blurred_button = Style::new()
            .foreground(dark_fg)
            .padding(&[0, 3]);
        s.blurred.title = Style::new().foreground(dark_fg);
        s.blurred.base = Style::new()
            .border(lipgloss_rs::border::rounded_border(), &[])
            .border_foreground(&[dark_fg]);
        s
    }
}

/// Catppuccin Mocha theme (dark).
pub struct CatppuccinTheme;
impl Theme for CatppuccinTheme {
    fn styles(&self, _is_dark: bool) -> Styles {
        // Catppuccin Mocha palette — hand-ported colour values.
        let rosewater = Color::parse("#f5e0dc");
        let flamingo  = Color::parse("#f2cdcd");
        let pink_     = Color::parse("#f5c2e7");
        let mauve     = Color::parse("#cba6f7");
        let red       = Color::parse("#f38ba8");
        let maroon    = Color::parse("#eba0ac");
        let peach     = Color::parse("#fab387");
        let yellow_   = Color::parse("#f9e2af");
        let green_    = Color::parse("#a6e3a1");
        let teal      = Color::parse("#94e2d5");
        let sky       = Color::parse("#89dceb");
        let sapphire  = Color::parse("#74c7ec");
        let _blue_    = Color::parse("#89b4fa");
        let lavender  = Color::parse("#b4befe");
        let text      = Color::parse("#cdd6f4");
        let subtext1  = Color::parse("#bac2de");
        let subtext0  = Color::parse("#a6adc8");
        let overlay2  = Color::parse("#9399b2");
        let overlay1  = Color::parse("#7f849c");
        let overlay0  = Color::parse("#6c7086");
        let surface2  = Color::parse("#585b70");
        let surface1  = Color::parse("#45475a");
        let surface0  = Color::parse("#313244");
        let base      = Color::parse("#1e1e2e");
        let mantle    = Color::parse("#181825");
        let crust     = Color::parse("#11111b");
        // suppress unused variable warnings for palette entries not yet used
        let _ = (rosewater, flamingo, pink_, red, maroon, peach, yellow_,
                 teal, sky, sapphire, lavender, subtext1, subtext0,
                 overlay2, overlay1, overlay0, surface1, surface0,
                 mantle, crust);

        let mut s = Styles::default();
        s.focused.title = Style::new().foreground(mauve).bold(true);
        s.focused.description = Style::new().foreground(surface2);
        s.focused.base = Style::new()
            .border(lipgloss_rs::border::rounded_border(), &[])
            .border_foreground(&[mauve]);
        s.focused.select_selector = Style::new().foreground(mauve);
        s.focused.multi_select_selector = Style::new().foreground(mauve);
        s.focused.selected_option = Style::new().foreground(green_);
        s.focused.selected_prefix = Style::new().foreground(green_);
        s.focused.unselected_option = Style::new().foreground(overlay2);
        s.focused.focused_button = Style::new()
            .bold(true)
            .foreground(base)
            .background(mauve)
            .padding(&[0, 3]);
        s.focused.blurred_button = Style::new()
            .foreground(overlay2)
            .padding(&[0, 3]);
        s.blurred.title = Style::new().foreground(overlay2);
        s.blurred.base = Style::new()
            .border(lipgloss_rs::border::rounded_border(), &[])
            .border_foreground(&[overlay0]);
        s.focused.text_input.text = Style::new().foreground(text);
        s.focused.text_input.placeholder = Style::new().foreground(overlay0);
        s.focused.text_input.prompt = Style::new().foreground(mauve);
        s
    }
}
