use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use adw::subclass::prelude::*;

use crate::models;
use crate::ui;

mod imp {
    use std::cell::{Cell, RefCell};

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type = super::StartView)]
    #[template(resource = "/com/github/manenfu/Khazanah/ui/start_view.ui")]
    pub struct StartView {
        #[template_child]
        pub start_controls: TemplateChild<ui::ToolbarStartControls>,
        #[template_child]
        pub end_controls: TemplateChild<ui::ToolbarEndControls>,

        #[property(get, set)]
        pub project_opened: Cell<bool>,
        #[property(get, set)]
        pub project_model: RefCell<models::ProjectModel>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for StartView {
        const NAME: &'static str = "KhzStartView";
        type Type = super::StartView;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for StartView {
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

    impl WidgetImpl for StartView {}
    impl BinImpl for StartView {}
}

glib::wrapper! {
    /// The first view when the application is started.
    pub struct StartView(ObjectSubclass<imp::StartView>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}
