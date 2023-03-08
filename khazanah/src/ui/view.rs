/// View that loads and commits its state to a model.
pub trait View {
    /// Load widget states from the project model.
    fn load_state(&self);

    /// Commit widget states to the project model.
    fn commit_state(&self);
}

/// Possible views of the main window.
/// Also check data/ui/toolbar_start_controls.ui
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum MainViews {
    Overview = 0,
    Phonology = 1,
    Lexicon = 2,

    Unknown = u32::MAX,
}

impl From<u32> for MainViews {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Overview,
            1 => Self::Phonology,
            2 => Self::Lexicon,
            _ => Self::Unknown,
        }
    }
}

impl From<MainViews> for u32 {
    fn from(value: MainViews) -> Self {
        value as Self
    }
}
