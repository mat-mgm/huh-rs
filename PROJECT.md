# Huh → Rust Port

## Overview
A port of [Huh](https://github.com/charmbracelet/huh) (`charm.land/huh/v2`,
Charm's interactive forms and prompts library) to Rust, built on top of the
sibling `bubbletea-rs`, `lipgloss-rs`, and `bubbles-rs` crates.

* **Purpose**: Provide idiomatic Rust form/prompt primitives — grouped fields,
  single-line input, multi-line text, single/multi-select, confirm dialogs,
  file picker, and note display — that compose into multi-step forms driven
  by the `bubbletea-rs` runtime.
* **Context**: Go source of truth lives at `../huh` (v2, ~7.6k LOC across 12
  source files). Dependencies that must be covered by the Rust port:
  * `bubbletea` / `lipgloss` / `bubbles` → covered by sibling crates.
  * `x/ansi` → already inside `lipgloss-rs::ansi`.
  * `catppuccin/go` → hand-port the relevant palette values for the Catppuccin
    theme; no external crate needed.
  * `mitchellh/hashstructure` → not needed; Rust equality/hashing is native.
  * `x/exp/ordered`, `x/exp/strings` → small utilities, inline as needed.
  * `x/term` → terminal size; use `bubbletea_rs::WindowSizeMsg` instead.
* **Scope**:
  * **In**: all six field types (Input, Text, Select, MultiSelect, Confirm,
    FilePicker), Note display, Group and Form orchestration, KeyMap, themes
    (Charm default, Base, Catppuccin), layout engine, accessibility mode
    (plain-text fallback), standalone runner and Bubble Tea embed mode.
  * **Out (for now)**: spinner field variant (`huh/spinner`), Windows
    pseudo-terminal testing harness (`x/xpty`).
* **References**:
  * Go source: `../huh/`
  * Sibling crates: `../bubbletea-rs`, `../lipgloss-rs`, `../bubbles-rs`
  * `SOURCES.md` — stack porting status

## Status
Current status: complete
Start date: 2026-05-31
Last updated: 2026-06-01
Priority: normal

## Goals
* **Functional parity** with Huh v2: every field type renders, validates, and
  submits correctly; forms navigate between groups; Bubble Tea embed works.
* **Idiomatic Rust**: fields are generic over their value type; validation is
  `Fn(&T) -> Result<(), String>`; builder pattern with consuming `with_*`
  setters.
* **Theme system**: Charm default, Base, and Catppuccin themes; custom themes
  via a `Theme` struct.
* **Accessible mode**: plain-text sequential prompt fallback (no TUI) for
  screen-reader users.

Success criteria:
* All field types compile and their unit tests pass.
* A representative multi-group form runs under `bubbletea-rs`.
* `cargo build`, `cargo test`, and `cargo clippy` are warning-free.

Constraints / priorities:
* Build on `bubbles-rs` components (textinput, textarea, filepicker) rather
  than re-implementing low-level editing.
* Do not modify upstream sibling crates.

## Architecture
* **Structure**: single crate `huh-rs`; one module per field type plus top-level
  `form`, `group`, `theme`, `keymap`, and `run` modules.
  ```
  src/
    lib.rs          — re-exports, Form, Group
    field/
      mod.rs        — Field trait
      input.rs      — single-line text input
      text.rs       — multi-line textarea
      select.rs     — single-choice select
      multiselect.rs— multi-choice select
      confirm.rs    — yes/no confirm
      filepicker.rs — file picker field
      note.rs       — static note/display
    keymap.rs       — KeyMap struct + default bindings
    theme.rs        — Theme, DefaultTheme, BaseTheme, CatppuccinTheme
    layout.rs       — column layout engine
    validate.rs     — ValidateFunc type alias + helpers
    run.rs          — standalone blocking runner
  ```
* **Key design decisions**:
  * `Field` trait: `update(&mut self, msg: &Msg) -> Option<Cmd>`,
    `view(&self) -> String`, `value(&self) -> &T`, `validate(&self) -> Result<(), String>`.
  * `Form` drives a sequence of `Group`s; each `Group` holds a `Vec<Box<dyn Field>>`.
  * Standalone mode: `Form::run()` boots a minimal `bubbletea-rs` program and
    blocks until submission or cancellation.
  * Bubble Tea embed: `Form` implements `bubbletea_rs::Model` so it can be
    nested inside a larger application.
* **Resources**: `bubbletea-rs` (path), `lipgloss-rs` (path), `bubbles-rs` (path).

## Development Guidelines
* Environment: Use Nix flakes (`nix develop`) to setup the dependencies. Enter the nix shell once and perform all development inside it.
* Version Control: Use `git` to track changes. Commit every time a new phase or feature is implemented and verified to work as expected.
* Workflow: After each phase implementation is done, wait for explicit user confirmation before marking the verification boxes in the roadmap and committing.
* Style: Use suckless coding style and robust coding practices.
* Warnings: Always address and fix compiler warnings.
* Commit convention: `Phase N: …`, authored as `dev <dev@localhost>`, no AI attribution.

## Roadmap

### General conditions
Go source of truth: `../huh/`. Sibling crates (`../bubbletea-rs`,
`../lipgloss-rs`, `../bubbles-rs`) are frozen — do not modify them; surface
gaps as separate tasks. One commit per phase.

Status legend: `[ ]` todo · `[~]` in-progress · `[✓]` done · `[x]` blocked ·
`[?]` optional · `[!]` critical.

### Phase 0: Scaffolding [✓]
**Description**: Create the `huh-rs` crate with path deps, module skeleton, git,
and Nix dev shell.

**Tasks**
- [✓] `Cargo.toml` with path deps on `bubbletea-rs`, `lipgloss-rs`, `bubbles-rs`.
- [✓] `src/lib.rs` module skeleton (empty modules for each field, form, group,
      keymap, theme, layout, validate, run).
- [✓] `flake.nix` dev shell (adapt from `bubbles-rs/flake.nix`).
- [✓] `git init` + initial commit.
- [✓] `.gitignore` to ignore build/target files.

**Checks**
- [✓] `cargo build` green on empty skeleton.
- [✓] `cargo clippy` warning-free.

### Phase 1: Field trait, KeyMap, Theme [✓]
**Description**: Define the `Field` trait and core supporting types.

**Tasks**
- [✓] `field/mod.rs` — `Field` trait with `update`, `view`, `focus`, `blur`,
      `is_done`, `validate`.
- [✓] `keymap.rs` — `KeyMap` struct mirroring `huh/keymap.go`; default bindings.
- [✓] `theme.rs` — `Theme` struct with style fields for each field state; implement
      Charm default, Base, and Catppuccin themes (hand-port palette values).
- [✓] `validate.rs` — `ValidateFunc<T>` type alias.

**Checks**
- [✓] Types compile; `cargo test` green.

**Dependencies**: Phase 0.

### Phase 2: Input and Text fields [✓]
**Description**: Port the two text-entry fields, wrapping `bubbles-rs` components.

**Tasks**
- [✓] `field/input.rs` — wraps `bubbles_rs::textinput::Model`; generic over
      `String`; supports placeholder, character limit, echo mode, validation,
      suggestions.
- [✓] `field/text.rs` — wraps `bubbles_rs::textarea::Model`; supports char limit,
      line limit, validation.
- [✓] Unit tests for basic input/submit/validation flows.

**Checks**
- [✓] Tests pass; fields render and accept input.

**Dependencies**: Phase 1.

### Phase 3: Select and MultiSelect fields [✓]
**Description**: Port the choice fields using the internal selector primitive.

**Tasks**
- [✓] `field/select.rs` — single-choice; options as `Vec<FieldOption<T>>` where
      `FieldOption<T>` is a label+value pair; keyboard navigation; filtering.
- [✓] `field/multiselect.rs` — multi-choice with limit; toggle selection; same
      navigation and filter as Select.
- [✓] Internal selector logic inlined into select/multiselect (no separate module needed).

**Checks**
- [✓] Tests pass; selection and filtering work correctly.

**Dependencies**: Phase 1.

### Phase 4: Confirm, Note, FilePicker fields [✓]
**Description**: Port the remaining three field types.

**Tasks**
- [✓] `field/confirm.rs` — yes/no toggle; affirmative/negative labels configurable.
- [✓] `field/note.rs` — static display-only field; title + body; next-on-submit.
- [✓] `field/filepicker.rs` — wraps `bubbles_rs::filepicker::Model`; allowed types;
      directory/file toggle.

**Checks**
- [✓] Tests pass; each field type behaves correctly.

**Dependencies**: Phase 2 (for consistency patterns).

### Phase 5: Group and Form orchestration [✓]
**Description**: Wire fields into groups and groups into a navigable form.

**Tasks**
- [✓] `group.rs` — `Group` holds `Vec<Box<dyn Field>>`; advances on all fields
      done; supports group-level title and description.
- [✓] `lib.rs` — `Form` holds `Vec<Group>`; drives navigation between
      groups; collects final values; returns on submit or cancel.
- [✓] Layout engine (`layout.rs`) — column layout for side-by-side fields.
- [✓] `run.rs` — `FormProgram` implements `bubbletea_rs::Model` (Bubble Tea embed mode).

**Checks**
- [✓] Multi-group form navigates correctly; values accessible after submit.

**Dependencies**: Phases 2–4.

### Phase 6: Standalone runner and accessibility mode [✓]
**Description**: Add the blocking `Form::run()` shortcut and the plain-text
accessibility fallback.

**Tasks**
- [✓] `Form::run()` boots a minimal `bubbletea-rs` program and blocks
      until the form is done; returns collected values.
- [✓] Accessibility mode — `Field::run_accessible()` trait method; each field
      prompts via `println!`/`stdin().read_line()`; `Form::run()` dispatches to
      `run_accessible()` when `accessible == true`.
- [✓] Expose `Form::groups()` and `Group::fields()` to allow programmatic inspection of fields.
- [✓] Add `Field::value_string()` (with implementation for `Input`) and `Select::selected_string()` to extract string values after a form run (especially for accessible mode).
- [✓] Make `Form::run_accessible` public and take `&mut self` so values are retained after running.

**Checks**
- [✓] `Form::run()` works in an end-to-end integration test.
- [✓] Accessibility mode produces correct output.

**Dependencies**: Phase 5.

### Phase 7: Examples and polish [✓]
**Description**: Add representative examples, final clippy sweep.

**Tasks**
- [✓] Examples: `simple` (single Input), `multi_group` (two groups with Select),
      `multiselect` (MultiSelect with limit).
- [✓] `cargo build --examples` green.
- [✓] `cargo test` warning-free (41 tests pass).

**Dependencies**: Phase 6.
