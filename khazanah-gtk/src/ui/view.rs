use std::fmt::{Debug, Display};

/// View that loads and commits its state to a model.
pub trait View {
    /// Loads widget states from the project model.
    fn load_state(&self) {}

    /// Commits widget states to the project model.
    fn commit_state(&self) {}

    /// Unloads widget states
    fn unload_state(&self) {}

    /// Called on window resize.
    fn on_window_size(&self, _width: i32, _height: i32) {}

    /// Connects things on headerbar.
    fn connect_headerbar(&self, _header_bar: &super::HeaderBar) {}
}

/// Possible views of the main window.
/// Also check data/ui/toolbar_start_controls.ui
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum MainView {
    Overview = 0,
    Phonology = 1,
    Lexicon = 2,

    #[default]
    Unknown = u32::MAX,
}

impl MainView {
    pub const ALL: &[Self] = &[Self::Overview, Self::Phonology, Self::Lexicon];
}

impl From<u32> for MainView {
    fn from(value: u32) -> Self {
        if let Some(mv) = Self::ALL.get(value as usize) {
            *mv
        } else {
            Self::Unknown
        }
    }
}

impl From<MainView> for u32 {
    fn from(value: MainView) -> Self {
        MainView::ALL
            .iter()
            .position(|v| v == &value)
            .unwrap_or_default() as u32
    }
}

impl Display for MainView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Overview => write!(f, "Overview"),
            Self::Phonology => write!(f, "Phonology"),
            Self::Lexicon => write!(f, "Lexicon"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}
