//! Widgets and UI components.

// Windows
mod window;

// Common Components
mod header_bar;
mod main_menu_button;
mod toolbar_end_controls;
mod toolbar_start_controls;
mod view_switcher_dropdown;

// Views
mod project_lexicon_view;
mod project_overview_view;
mod start_view;
mod view;

pub use header_bar::HeaderBar;
pub use main_menu_button::MainMenuButton;
pub use project_lexicon_view::ProjectLexiconView;
pub use project_overview_view::ProjectOverviewView;
pub use start_view::StartView;
pub use toolbar_end_controls::ToolbarEndControls;
pub use toolbar_start_controls::ToolbarStartControls;
pub use view::{MainViews, View, ALL_MAIN_VIEWS};
pub use view_switcher_dropdown::ViewSwitcherDropDown;
pub use window::ApplicationWindow;
