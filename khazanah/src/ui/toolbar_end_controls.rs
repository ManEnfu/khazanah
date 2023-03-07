use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use adw::subclass::prelude::*;

mod imp {
    use std::cell::Cell;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type = super::ToolbarEndControls)]
    #[template(resource = "/com/github/manenfu/Khazanah/ui/toolbar_end_controls.ui")]
    pub struct ToolbarEndControls {
        #[template_child]
        pub save_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub save_as_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub tool_menu_button: TemplateChild<gtk::MenuButton>,
        #[template_child]
        pub main_menu_button: TemplateChild<gtk::MenuButton>,

        #[property(get, set)]
        pub buttons_sensitive: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ToolbarEndControls {
        const NAME: &'static str = "KhzToolbarEndControls";
        type Type = super::ToolbarEndControls;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ToolbarEndControls {
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

    impl WidgetImpl for ToolbarEndControls {}
    impl BinImpl for ToolbarEndControls {}
}

glib::wrapper! {
    /// Control widgets at the end of the toolbar.
    pub struct ToolbarEndControls(ObjectSubclass<imp::ToolbarEndControls>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}