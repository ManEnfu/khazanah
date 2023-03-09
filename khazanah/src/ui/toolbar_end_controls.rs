use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use adw::subclass::prelude::*;

mod imp {
    use std::cell::Cell;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/manenfu/Khazanah/ui/toolbar_end_controls.ui")]
    pub struct ToolbarEndControls {
        #[template_child]
        pub save_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub save_as_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub tool_menu_button: TemplateChild<gtk::MenuButton>,
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

    impl ObjectImpl for ToolbarEndControls {}

    impl WidgetImpl for ToolbarEndControls {}
    impl BinImpl for ToolbarEndControls {}
}

glib::wrapper! {
    /// Control widgets at the end of the toolbar.
    pub struct ToolbarEndControls(ObjectSubclass<imp::ToolbarEndControls>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}
