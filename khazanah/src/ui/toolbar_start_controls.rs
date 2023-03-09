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
        // #[template_child]
        // pub view_dropdown: TemplateChild<gtk::DropDown>,
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

    impl ObjectImpl for ToolbarStartControls {
        // fn constructed(&self) {
        //     self.parent_constructed();

        //     // Load view dropdown model.
        //     let main_views =
        //         gtk::Builder::from_resource("/com/github/manenfu/Khazanah/ui/main_views.ui")
        //             .object::<gtk::StringList>("main_views")
        //             .expect("Loading nonexistent model `main_views` from resource.");
        //     self.view_dropdown.set_model(Some(&main_views));
        // }

        // fn properties() -> &'static [glib::ParamSpec] {
        //     Self::derived_properties()
        // }

        // fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        //     self.derived_set_property(id, value, pspec)
        // }

        // fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        //     self.derived_property(id, pspec)
        // }
    }

    impl WidgetImpl for ToolbarStartControls {}
    impl BinImpl for ToolbarStartControls {}
}

glib::wrapper! {
    /// Control widgets at the start of the toolbar.
    pub struct ToolbarStartControls(ObjectSubclass<imp::ToolbarStartControls>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}
