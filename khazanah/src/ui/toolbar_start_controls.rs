use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use adw::subclass::prelude::*;

mod imp {
    use std::cell::Cell;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/manenfu/Khazanah/ui/toolbar_start_controls.ui")]
    pub struct ToolbarStartControls {
        #[template_child]
        pub open_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub new_button: TemplateChild<gtk::Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ToolbarStartControls {
        const NAME: &'static str = "KhzToolbarStartControls";
        type Type = super::ToolbarStartControls;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ToolbarStartControls {}

    impl WidgetImpl for ToolbarStartControls {}
    impl BinImpl for ToolbarStartControls {}
}

glib::wrapper! {
    /// Control widgets at the start of the toolbar.
    pub struct ToolbarStartControls(ObjectSubclass<imp::ToolbarStartControls>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}
