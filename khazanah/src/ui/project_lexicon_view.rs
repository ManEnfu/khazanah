use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use adw::prelude::*;
use adw::subclass::prelude::*;

use crate::models;
use crate::ui;

mod imp {
    use std::cell::{Cell, RefCell};

    use conlang::ALL_PARTS_OF_SPEECH;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type = super::ProjectLexiconView)]
    #[template(resource = "/com/github/manenfu/Khazanah/ui/project_lexicon_view.ui")]
    pub struct ProjectLexiconView {
        #[template_child]
        pub pos_dropdown: TemplateChild<adw::ComboRow>,

        #[property(get, set)]
        pub project_model: RefCell<models::ProjectModel>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ProjectLexiconView {
        const NAME: &'static str = "KhzProjectLexiconView";
        type Type = super::ProjectLexiconView;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ProjectLexiconView {
        fn constructed(&self) {
            self.parent_constructed();

            let pos_list: Vec<&str> = ALL_PARTS_OF_SPEECH.iter().map(|pos| pos.name()).collect();
            let pos_model = gtk::StringList::new(&pos_list);
            self.pos_dropdown.set_model(Some(&pos_model));
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

    impl WidgetImpl for ProjectLexiconView {}
    impl BinImpl for ProjectLexiconView {}
}

glib::wrapper! {
    /// The view to edit project lexicon.
    pub struct ProjectLexiconView(ObjectSubclass<imp::ProjectLexiconView>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

#[gtk::template_callbacks]
impl ProjectLexiconView {}

impl ui::View for ProjectLexiconView {
    fn load_state(&self) {
        let dirty = self.project_model().dirty();

        self.project_model().set_dirty(dirty);
    }

    // fn commit_state(&self) {
    //     // let imp = self.imp();

    //     self.project_model().set_dirty(true);
    // }
}
