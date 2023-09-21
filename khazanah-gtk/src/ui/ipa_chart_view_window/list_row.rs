use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

// use adw::prelude::*;
use adw::subclass::prelude::*;

use crate::models::PhonemeObject;

#[doc(hidden)]
mod imp {
    use std::cell::RefCell;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type = super::ListRow)]
    #[template(resource = "/com/github/manenfu/Khazanah/ui/ipa_chart_view_window/list_row.ui")]
    pub struct ListRow {
        #[template_child]
        pub name_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub symbol_label: TemplateChild<gtk::Label>,

        #[property(get, set)]
        pub phoneme: RefCell<Option<PhonemeObject>>,

        pub bindings: RefCell<Vec<glib::Binding>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ListRow {
        const NAME: &'static str = "KhzIpaChartViewWindowListRow";
        type Type = super::ListRow;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ListRow {
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

    impl WidgetImpl for ListRow {}

    impl BinImpl for ListRow {}
}

glib::wrapper! {
    pub struct ListRow(ObjectSubclass<imp::ListRow>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

#[gtk::template_callbacks]
impl ListRow {
    /// Creates a new list row.
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}

impl Default for ListRow {
    fn default() -> Self {
        Self::new()
    }
}
