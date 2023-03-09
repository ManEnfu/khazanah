use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use adw::subclass::prelude::*;

mod imp {
    use std::cell::Cell;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type = super::ViewSwitcherDropDown)]
    #[template(resource = "/com/github/manenfu/Khazanah/ui/view_switcher_dropdown.ui")]
    pub struct ViewSwitcherDropDown {
        #[template_child]
        pub view_dropdown: TemplateChild<gtk::DropDown>,

        #[property(get, set)]
        pub selected_view_index: Cell<u32>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ViewSwitcherDropDown {
        const NAME: &'static str = "KhzViewSwitcherDropDown";
        type Type = super::ViewSwitcherDropDown;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ViewSwitcherDropDown {
        // fn constructed(&self) {
        //     self.parent_constructed();

        //     // Load view dropdown model.
        //     let main_views =
        //         gtk::Builder::from_resource("/com/github/manenfu/Khazanah/ui/main_views.ui")
        //             .object::<gtk::StringList>("main_views")
        //             .expect("Loading nonexistent model `main_views` from resource.");
        //     self.view_dropdown.set_model(Some(&main_views));
        // }

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

    impl WidgetImpl for ViewSwitcherDropDown {}
    impl BinImpl for ViewSwitcherDropDown {}
}

glib::wrapper! {
    /// Control widgets at the start of the toolbar.
    pub struct ViewSwitcherDropDown(ObjectSubclass<imp::ViewSwitcherDropDown>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}
