use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use adw::subclass::prelude::*;

#[allow(clippy::enum_variant_names)]
mod imp {
    use std::cell::RefCell;

    use gtk::glib::subclass::Signal;
    use once_cell::sync::Lazy;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type = super::TextAreaRow)]
    #[template(resource = "/com/github/manenfu/Khazanah/ui/text_area_row.ui")]
    pub struct TextAreaRow {
        #[template_child]
        pub text_view: TemplateChild<gtk::TextView>,

        #[property(get, set)]
        pub title: RefCell<String>,
        #[property(get, set)]
        pub buffer: RefCell<gtk::TextBuffer>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TextAreaRow {
        const NAME: &'static str = "KhzTextAreaRow";
        type Type = super::TextAreaRow;
        type ParentType = adw::PreferencesRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for TextAreaRow {
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

    impl WidgetImpl for TextAreaRow {}
    impl ListBoxRowImpl for TextAreaRow {}
    impl PreferencesRowImpl for TextAreaRow {}
}

glib::wrapper! {
    pub struct TextAreaRow(ObjectSubclass<imp::TextAreaRow>)
        @extends gtk::Widget, gtk::ListBoxRow, adw::PreferencesRow,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

#[gtk::template_callbacks]
impl TextAreaRow {}
