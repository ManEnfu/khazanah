use adw::subclass::prelude::*;
use gtk::glib;
use gtk::prelude::*;

use crate::models;
use crate::ui;

pub use content::Content;
pub use sidebar::Sidebar;

mod content;
mod sidebar;

#[doc(hidden)]
#[allow(clippy::enum_variant_names)]
mod imp {
    use std::cell::RefCell;

    use gtk::glib::subclass::Signal;
    use once_cell::sync::Lazy;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type = super::InventoryView)]
    #[template(resource = "/com/github/manenfu/Khazanah/ui/view/inventory.ui")]
    pub struct InventoryView {
        #[template_child]
        pub leaflet: TemplateChild<adw::Leaflet>,

        #[template_child]
        pub sidebar: TemplateChild<Sidebar>,
        #[template_child]
        pub content: TemplateChild<Content>,

        #[property(get, set)]
        pub project_model: RefCell<models::ProjectModel>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for InventoryView {
        const NAME: &'static str = "KhzInventoryView";
        type Type = super::InventoryView;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for InventoryView {
        fn constructed(&self) {
            self.parent_constructed();
        }

        fn properties() -> &'static [glib::ParamSpec] {
            Self::derived_properties()
        }

        fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            self.derived_set_property(id, value, pspec)
        }

        fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            self.derived_property(id, pspec)
        }

        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(Vec::new);
            SIGNALS.as_ref()
        }
    }

    impl WidgetImpl for InventoryView {}
    impl BinImpl for InventoryView {}
}

glib::wrapper! {
    pub struct InventoryView(ObjectSubclass<imp::InventoryView>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

#[gtk::template_callbacks]
impl InventoryView {}

impl ui::View for InventoryView {}
