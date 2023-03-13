use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use adw::subclass::prelude::*;

use crate::ui;

mod imp {
    use std::cell::Cell;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type = super::HeaderBar)]
    #[template(resource = "/com/github/manenfu/Khazanah/ui/header_bar.ui")]
    pub struct HeaderBar {
        #[template_child]
        pub header_bar: TemplateChild<adw::HeaderBar>,

        // Start
        #[template_child]
        pub back_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub view_switcher: TemplateChild<ui::ViewSwitcherDropDown>,

        // Center
        // #[template_child]
        // pub title_widget: TemplateChild<adw::WindowTitle>,

        // End
        #[template_child]
        pub start_controls: TemplateChild<ui::ToolbarStartControls>,
        #[template_child]
        pub end_controls: TemplateChild<ui::ToolbarEndControls>,
        #[template_child]
        pub main_menu_button: TemplateChild<ui::MainMenuButton>,

        #[template_child]
        pub search_word_button: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub add_word_button: TemplateChild<gtk::Button>,

        #[property(get, set)]
        pub selected_view_index: Cell<u32>,
        #[property(get, set)]
        pub view_switcher_sensitive: Cell<bool>,

        #[property(get, set)]
        pub reveal_back_button: Cell<bool>,
        #[property(get, set)]
        pub reveal_toolbar_buttons: Cell<bool>,

        #[property(get, set)]
        pub reveal_lexicon_view_buttons: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for HeaderBar {
        const NAME: &'static str = "KhzHeaderBar";
        type Type = super::HeaderBar;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for HeaderBar {
        fn properties() -> &'static [glib::ParamSpec] {
            Self::derived_properties()
        }

        fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            self.derived_set_property(id, value, pspec)
        }

        fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            self.derived_property(id, pspec)
        }
    }

    impl WidgetImpl for HeaderBar {}
    impl BinImpl for HeaderBar {}
}

glib::wrapper! {
    /// Control widgets at the end of the toolbar.
    pub struct HeaderBar(ObjectSubclass<imp::HeaderBar>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl HeaderBar {
    pub fn set_flat(&self, value: bool) {
        let hb = &self.imp().header_bar;
        if value {
            hb.add_css_class("flat");
        } else {
            hb.remove_css_class("flat");
        }
    }
}
