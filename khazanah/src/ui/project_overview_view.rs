use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use adw::subclass::prelude::*;

use crate::models;
use crate::ui;

#[doc(hidden)]
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

        #[property(get, set)]
        pub meta_object: RefCell<models::MetaObject>,

        pub form_bindings: RefCell<Vec<glib::Binding>>,
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
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup();
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
    fn setup(&self) {
        self.bind_property("project-model", &self.meta_object(), "project-model")
            .sync_create()
            .build();
    }
}

impl ui::View for ProjectOverviewView {
    fn load_state(&self) {
        log::debug!("Loading view state.");

        let imp = self.imp();
        let dirty = self.project_model().dirty();

        let mut bindings = imp.form_bindings.borrow_mut();
        let meta_object = self.meta_object();

        for binding in bindings.drain(..) {
            binding.unbind()
        }

        bindings.push(
            meta_object
                .bind_property("name", &imp.lang_family_name_entry.get(), "text")
                .sync_create()
                .bidirectional()
                .build(),
        );

        bindings.push(
            meta_object
                .bind_property("local-language", &imp.local_lang_entry.get(), "text")
                .sync_create()
                .bidirectional()
                .build(),
        );

        bindings.push(
            meta_object
                .bind_property("author", &imp.author_entry.get(), "text")
                .sync_create()
                .bidirectional()
                .build(),
        );

        bindings.push(
            meta_object
                .bind_property("description", &imp.description_entry.get(), "text")
                .sync_create()
                .bidirectional()
                .build(),
        );

        self.project_model().set_dirty(dirty);
    }

    fn commit_state(&self) {
        log::debug!("Committing view state.");

        let imp = self.imp();

        for binding in imp.form_bindings.borrow_mut().drain(..) {
            binding.unbind()
        }

        self.project_model().set_dirty(true);
    }
}
