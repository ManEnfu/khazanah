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
    #[properties(wrapper_type = super::ProjectPhonologyView)]
    #[template(resource = "/com/github/manenfu/Khazanah/ui/project_phonology_view.ui")]
    pub struct ProjectPhonologyView {
        #[template_child]
        pub add_phoneme_button: TemplateChild<ui::AddPhonemeButton>,

        #[property(get, set)]
        pub project_model: RefCell<models::ProjectModel>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ProjectPhonologyView {
        const NAME: &'static str = "KhzProjectPhonologyView";
        type Type = super::ProjectPhonologyView;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ProjectPhonologyView {
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

    impl WidgetImpl for ProjectPhonologyView {}

    impl BinImpl for ProjectPhonologyView {}
}

glib::wrapper! {
    /// The first view when the application is started.
    pub struct ProjectPhonologyView(ObjectSubclass<imp::ProjectPhonologyView>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

#[gtk::template_callbacks]
impl ProjectPhonologyView {
    #[template_callback]
    fn handle_add_phoneme_button_selected(
        &self,
        phoneme_object: &models::AddPhonemeObject,
        _button: ui::AddPhonemeButton,
    ) {
        log::debug!("Add phoneme: {}", phoneme_object.name());
    }
}

impl ui::View for ProjectPhonologyView {}
