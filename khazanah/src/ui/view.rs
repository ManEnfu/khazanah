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
pub enum MainViews {
    Overview = 0,
    Phonology = 1,
    Lexicon = 2,

    #[default]
    Unknown = u32::MAX,
}

pub const ALL_MAIN_VIEWS: &[MainViews] = &[MainViews::Overview, MainViews::Lexicon];

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
