use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use adw::subclass::prelude::*;

use crate::models;
use crate::ui;

mod imp {
    use std::cell::RefCell;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type = super::ProjectOverviewView)]
    #[template(resource = "/com/github/manenfu/Khazanah/ui/project_overview_view.ui")]
    pub struct ProjectOverviewView {
        #[template_child]
        pub lang_family_name_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub local_lang_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub author_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub description_entry: TemplateChild<adw::EntryRow>,

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
    fn handle_lang_family_name_entry_apply(&self, _entry: &adw::EntryRow) {
        self.project_model().set_dirty(true);
    }

    #[template_callback]
    fn handle_author_entry_apply(&self, _entry: &adw::EntryRow) {
        self.project_model().set_dirty(true);
    }

    #[template_callback]
    fn handle_description_entry_apply(&self, _entry: &adw::EntryRow) {
        self.project_model().set_dirty(true);
    }

    #[template_callback]
    fn handle_local_lang_entry_apply(&self, _entry: &adw::EntryRow) {
        self.project_model().set_dirty(true);
    }
}

impl ui::View for ProjectOverviewView {
    fn load_state(&self) {
        let imp = self.imp();
        let dirty = self.project_model().dirty();

        if let Some(project) = self.project_model().project().as_ref() {
            let meta = project.meta();
            imp.lang_family_name_entry.set_text(&meta.name);
            imp.local_lang_entry.set_text(&meta.local_lang);
            imp.author_entry.set_text(&meta.author);
            imp.description_entry.set_text(&meta.description);
        }

        self.project_model().set_dirty(dirty);
    }

    fn commit_state(&self) {
        let imp = self.imp();

        if let Some(project) = self.project_model().project_mut().as_mut() {
            let meta = project.meta_mut();
            meta.name = imp.lang_family_name_entry.text().to_string();
            meta.local_lang = imp.local_lang_entry.text().to_string();
            meta.author = imp.author_entry.text().to_string();
            meta.description = imp.description_entry.text().to_string();
        }

        self.project_model().set_dirty(true);
    }
}
