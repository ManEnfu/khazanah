use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gdk, glib};

use adw::subclass::prelude::*;

use crate::models;
use crate::ui;

pub use word_edit_view::ProjectLexiconWordEditView;
pub use word_list_row::ProjectLexiconWordListRow;
pub use word_list_view::ProjectLexiconWordListView;

mod word_edit_view;
mod word_list_row;
mod word_list_view;

#[doc(hidden)]
mod imp {
    use std::cell::RefCell;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type = super::ProjectLexiconView)]
    #[template(resource = "/com/github/manenfu/Khazanah/ui/project_lexicon_view.ui")]
    pub struct ProjectLexiconView {
        #[template_child]
        pub leaflet: TemplateChild<adw::Leaflet>,

        #[template_child]
        pub word_list_view: TemplateChild<ProjectLexiconWordListView>,
        #[template_child]
        pub word_edit_view: TemplateChild<ProjectLexiconWordEditView>,

        #[property(get, set)]
        pub project_model: RefCell<models::ProjectModel>,

        pub header_bar: RefCell<Option<ui::HeaderBar>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ProjectLexiconView {
        const NAME: &'static str = "KhzProjectLexiconView";
        type Type = super::ProjectLexiconView;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();

            klass.install_action("lexicon.go-back", None, move |view, _, _| {
                view.navigate_back();
            });

            klass.add_binding_action(
                gdk::Key::Escape,
                gdk::ModifierType::empty(),
                "lexicon.go-back",
                None,
            );
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ProjectLexiconView {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();
            obj.setup_callbacks();
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
impl ProjectLexiconView {
    /// Setups callbacks.
    fn setup_callbacks(&self) {
        let imp = self.imp();

        imp.word_list_view.connect_closure(
            "word-selected",
            false,
            glib::closure_local!(@strong self as view => move |_: &ProjectLexiconWordListView| {
                view.load_selected_word();
            }),
        );

        imp.word_list_view.connect_closure(
            "word-activated",
            false,
            glib::closure_local!(@strong self as view => move |_: &ProjectLexiconWordListView| {
                view.handle_activate_word();
            }),
        );

        imp.word_list_view.connect_closure(
            "search-changed",
            false,
            glib::closure_local!(@strong self as view => move |_: &ProjectLexiconWordListView| {
                view.load_selected_word();
            }),
        );

        imp.leaflet.connect_notify_local(
            Some("folded"),
            glib::clone!(@weak self as view => move |_leaflet, _| {
                view.update_buttons_visibility();
            }),
        );
    }

    /// Loads form contents with word data.
    pub fn load_selected_word(&self) {
        let imp = self.imp();

        imp.word_edit_view.unbind();
        let word = imp.word_list_view.selected_word();

        if let Some(word) = word {
            log::debug!("selected word: {}", word.id());
            imp.word_edit_view.bind(&word);
            imp.word_edit_view.set_fields_sensitive(true);
        } else {
            imp.word_edit_view.clear_fields();
            imp.word_edit_view.set_fields_sensitive(false);
        }

        self.navigate_back();
    }

    /// Activates currently selected word. If the leaflet is folded, switch to form page.
    fn handle_activate_word(&self) {
        let imp = self.imp();

        if imp.leaflet.is_folded() {
            self.navigate_to_forward();
        }
    }

    /// Navigates forward.
    fn navigate_to_forward(&self) {
        let imp = self.imp();
        imp.leaflet.navigate(adw::NavigationDirection::Forward);
        self.update_buttons_visibility();
    }

    /// Navigates back.
    fn navigate_back(&self) {
        let imp = self.imp();
        imp.leaflet.navigate(adw::NavigationDirection::Back);
        if imp.leaflet.is_folded() {
            if let Some(word) = imp.word_list_view.selected_word() {
                imp.word_list_view.notify_changes_to_model(&word);
            }
        }
        self.update_buttons_visibility();
    }

    /// Updates visibility of some buttons.
    fn update_buttons_visibility(&self) {
        let imp = self.imp();
        if let Some(header_bar) = imp.header_bar.borrow().as_ref() {
            header_bar.set_reveal_back_button(
                imp.leaflet.is_folded()
                    && imp.leaflet.visible_child_name() != Some("word-list-view".into()),
            );
        }
    }
}

impl ui::View for ProjectLexiconView {
    fn load_state(&self) {
        log::debug!("Loading view state.");

        let imp = self.imp();

        imp.word_list_view.load_state();
        imp.word_edit_view.load_state();

        self.load_selected_word();
    }

    fn commit_state(&self) {
        log::debug!("Committing view state.");

        let imp = self.imp();
        imp.word_list_view.commit_state();
        imp.word_edit_view.commit_state();
    }

    fn connect_headerbar(&self, header_bar: &ui::HeaderBar) {
        let imp = self.imp();

        header_bar
            .imp()
            .back_button
            .connect_clicked(glib::clone!(@weak self as view => move |_| {
                view.activate_action("lexicon.go-back", None).unwrap_or_default();
            }));

        imp.header_bar.replace(Some(header_bar.clone()));
    }
}
