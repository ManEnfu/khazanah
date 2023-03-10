//! Widgets and UI components.

// Windows
mod window;

// Common Components
mod toolbar_end_controls;
mod toolbar_start_controls;
mod main_menu_button; 
mod view_switcher_dropdown;
mod header_bar;

// Views
mod project_lexicon_view;
mod project_overview_view;
mod start_view;
mod view;

pub use main_menu_button::MainMenuButton;
pub use project_lexicon_view::ProjectLexiconView;
pub use project_overview_view::ProjectOverviewView;
pub use start_view::StartView;
pub use toolbar_end_controls::ToolbarEndControls;
pub use toolbar_start_controls::ToolbarStartControls;
pub use view::{MainViews, View, ALL_MAIN_VIEWS};
pub use view_switcher_dropdown::ViewSwitcherDropDown;
pub use window::ApplicationWindow;
pub use header_bar::HeaderBar;
