# huh-rs

A native Rust port of [Huh?](https://github.com/charmbracelet/huh), an interactive forms and prompts library for the terminal.

## Features

- Diverse input fields:
  - `Input`: Single-line text input with prompt, placeholder, and validation.
  - `Text`: Multi-line text entry with navigation and line counting.
  - `Select`: Single option picker from a list with fuzzy filtering.
  - `MultiSelect`: Multiple option selector with toggle, filtering, and selection limits.
  - `Confirm`: Yes/No binary prompt.
  - `FilePicker`: Interactive filesystem path picker.
  - `Note`: Informational display banner with Markdown/text support.
- Multi-step form orchestration: compose fields into sequential `Group` containers within a `Form`.
- Dynamic validation: custom closure validators (`Fn(&T) -> Result<(), String>`) with error feedback.
- Customizable theming: built-in Charm default, Base, and Catppuccin themes.
- Accessible mode: optional non-TUI line-by-line fallback for screen readers and dumb terminals.
- Standalone execution or embeddable directly as a `bubbletea_rs::Model`.

## Installation

Add `huh-rs` and its dependencies to your `Cargo.toml`:

```toml
[dependencies]
huh-rs = "0.1.0"
bubbletea-rs = "0.1.0"
lipgloss-rs = "0.1.0"
bubbles-rs = "0.1.0"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

## Quick Start

```rust
use huh_rs::{field, field::input::Input, group, Form};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let name_field = Input::new()
        .with_title("Name")
        .with_description("Enter your full name")
        .with_key("name");

    let form = Form::new(vec![
        group(vec![field!(name_field)]),
    ]);

    form.run()?;

    println!("Form completed successfully.");
    Ok(())
}
```

## Multi-Field and Multi-Group Forms

```rust
use huh_rs::{
    field,
    field::{confirm::Confirm, input::Input, select::Select},
    group, Form, OptionItem,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let form = Form::new(vec![
        group(vec![
            field!(Input::new()
                .with_title("Project Name")
                .with_key("project")),
            field!(Select::new()
                .with_title("Choose Language")
                .with_options(vec![
                    OptionItem::new("Rust", "rust"),
                    OptionItem::new("Go", "go"),
                    OptionItem::new("Zig", "zig"),
                ])
                .with_key("lang")),
        ]),
        group(vec![
            field!(Confirm::new()
                .with_title("Initialize git repository?")
                .with_key("git")),
        ]),
    ]);

    form.run()?;
    Ok(())
}
```

## Examples

Run the included examples with Cargo:

```bash
cargo run --example simple
cargo run --example multi_group
cargo run --example multiselect
```

## License

This project is licensed under the MIT License.
