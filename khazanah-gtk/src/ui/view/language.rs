use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use adw::subclass::prelude::*;

use crate::models;
use crate::ui;

#[doc(hidden)]
pub mod imp {
    use std::cell::RefCell;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type = super::LanguageView)]
    #[template(resource = "/com/github/manenfu/Khazanah/ui/view/language.ui")]
    pub struct LanguageView {
        #[template_child]
        pub lang_name_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub local_lang_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub author_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub description_area: TemplateChild<ui::TextAreaRow>,

        #[property(get, set)]
        pub project_model: RefCell<models::ProjectModel>,

        #[property(get, set)]
        pub meta_object: RefCell<models::MetaObject>,

        pub form_bindings: RefCell<Vec<glib::Binding>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for LanguageView {
        const NAME: &'static str = "KhzLanguageView";
        type Type = super::LanguageView;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for LanguageView {
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

    impl WidgetImpl for LanguageView {}
    impl BinImpl for LanguageView {}
}

glib::wrapper! {
    /// The view to edit project general data, such as name and description.
    pub struct LanguageView(ObjectSubclass<imp::LanguageView>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

#[gtk::template_callbacks]
impl LanguageView {
    fn setup(&self) {
        self.bind_property("project-model", &self.meta_object(), "project-model")
            .sync_create()
            .build();
    }
}

impl ui::View for LanguageView {
    fn load_state(&self) {
        log::debug!("Loading view state.");

        let imp = self.imp();

        let mut bindings = imp.form_bindings.borrow_mut();
        let meta_object = self.meta_object();

        for binding in bindings.drain(..) {
            binding.unbind()
        }

        bindings.push(
            meta_object
                .bind_property("name", &imp.lang_name_entry.get(), "text")
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

        let desc = meta_object.description();
        imp.description_area.buffer().set_text(&desc);
    }

    fn commit_state(&self) {
        let desc_buf = self.imp().description_area.buffer();
        let desc = desc_buf.text(&desc_buf.start_iter(), &desc_buf.end_iter(), false);

        self.meta_object().set_description(desc);
    }

    fn unload_state(&self) {
        log::debug!("Unloading view state.");

        let imp = self.imp();

        for binding in imp.form_bindings.borrow_mut().drain(..) {
            binding.unbind()
        }
    }
}
