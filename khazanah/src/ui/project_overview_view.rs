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
    #[properties(wrapper_type = super::ProjectOverviewView)]
    #[template(resource = "/com/github/manenfu/Khazanah/ui/project_overview_view.ui")]
    pub struct ProjectOverviewView {
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
    impl ObjectSubclass for ProjectOverviewView {
        const NAME: &'static str = "KhzProjectOverviewView";
        type Type = super::ProjectOverviewView;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ProjectOverviewView {
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

    impl WidgetImpl for ProjectOverviewView {}
    impl BinImpl for ProjectOverviewView {}
}

glib::wrapper! {
    /// The view to edit project general data, such as name and description.
    pub struct ProjectOverviewView(ObjectSubclass<imp::ProjectOverviewView>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

#[gtk::template_callbacks]
impl ProjectOverviewView {
    #[template_callback]
    fn handle_lang_family_name_entry_apply(&self, entry: &adw::EntryRow) {
        let s = entry.text();
        log::debug!("lang_family_name_entry::apply({})", &s);
    }

    #[template_callback]
    fn handle_author_entry_apply(&self, entry: &adw::EntryRow) {
        let s = entry.text();
        log::debug!("author_entry::apply({})", &s);
    }

    #[template_callback]
    fn handle_description_entry_apply(&self, entry: &adw::EntryRow) {
        let s = entry.text();
        log::debug!("description_entry::apply({})", &s);
    }
}
