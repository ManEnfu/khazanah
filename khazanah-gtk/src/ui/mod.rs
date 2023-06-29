//! Widgets and UI components.

// Windows
mod window;

// Common Components
mod add_phoneme_button;
mod add_phoneme_row;
mod header_bar;
mod ipa_chart;
mod main_menu_button;
mod toolbar_end_controls;
mod toolbar_start_controls;
mod view_switcher_dropdown;

// Views
mod project_lexicon_view;
mod project_overview_view;
mod project_phonology_view;
mod start_view;
mod view;

mod ipa_chart_view_window;
mod xsampa_view_window;

pub use add_phoneme_button::AddPhonemeButton;
pub use add_phoneme_row::AddPhonemeRow;
pub use header_bar::HeaderBar;
pub use ipa_chart::IpaChart;
pub use ipa_chart_view_window::IpaChartViewWindow;
pub use main_menu_button::MainMenuButton;
pub use project_lexicon_view::ProjectLexiconView;
pub use project_overview_view::ProjectOverviewView;
pub use project_phonology_view::ProjectPhonologyView;
pub use start_view::StartView;
pub use toolbar_end_controls::ToolbarEndControls;
pub use toolbar_start_controls::ToolbarStartControls;
pub use view::{MainView, View};
pub use view_switcher_dropdown::ViewSwitcherDropDown;
pub use window::ApplicationWindow;
pub use xsampa_view_window::XSampaViewWindow;
