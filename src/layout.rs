/// Column layout — places fields side-by-side in columns.
/// A `Layout::default()` stacks fields vertically (single column).
#[derive(Debug, Clone, Default)]
pub enum Layout {
    /// Fields stacked vertically (default).
    #[default]
    Stack,
    /// Fields arranged in N equal-width columns.
    Columns(usize),
}

impl Layout {
    /// Compute per-column width given total available width.
    pub fn column_width(&self, total_width: usize) -> usize {
        match self {
            Layout::Stack       => total_width,
            Layout::Columns(n)  => if *n == 0 { total_width } else { total_width / n },
        }
    }
}
