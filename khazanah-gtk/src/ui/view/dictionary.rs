use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gdk, glib};

use adw::subclass::prelude::*;

use crate::models;
use crate::ui;

pub use content::Content;
pub use sidebar::Sidebar;
pub use word_list_row::WordListRow;

mod content;
mod sidebar;
mod word_list_row;

#[doc(hidden)]
mod imp {
    use std::cell::RefCell;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type = super::DictionaryView)]
    #[template(resource = "/com/github/manenfu/Khazanah/ui/view/dictionary.ui")]
    pub struct DictionaryView {
        #[template_child]
        pub leaflet: TemplateChild<adw::Leaflet>,

        #[template_child]
        pub sidebar: TemplateChild<Sidebar>,
        #[template_child]
        pub content: TemplateChild<Content>,

        #[property(get, set)]
        pub project_model: RefCell<models::ProjectModel>,

        pub header_bar: RefCell<Option<ui::HeaderBar>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DictionaryView {
        const NAME: &'static str = "KhzDictionaryView";
        type Type = super::DictionaryView;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();

            klass.install_action("dictionary.go-back", None, move |view, _, _| {
                view.navigate_back();
            });

            klass.add_binding_action(
                gdk::Key::Escape,
                gdk::ModifierType::empty(),
                "dictionary.go-back",
                None,
            );
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for DictionaryView {
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

    impl WidgetImpl for DictionaryView {}
    impl BinImpl for DictionaryView {}
}

glib::wrapper! {
    /// The view to edit project dictionary.
    pub struct DictionaryView(ObjectSubclass<imp::DictionaryView>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

#[gtk::template_callbacks]
impl DictionaryView {
    /// Setups callbacks.
    fn setup_callbacks(&self) {
        let imp = self.imp();

        imp.sidebar.connect_closure(
            "word-selected",
            false,
            glib::closure_local!(@strong self as view => move |_: &Sidebar| {
                view.load_selected_word();
            }),
        );

        imp.sidebar.connect_closure(
            "word-activated",
            false,
            glib::closure_local!(@strong self as view => move |_: &Sidebar| {
                view.handle_activate_word();
            }),
        );

        imp.sidebar.connect_closure(
            "search-changed",
            false,
            glib::closure_local!(@strong self as view => move |_: &Sidebar| {
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

        imp.content.unbind();
        let word = imp.sidebar.selected_word();

        if let Some(word) = word {
            log::debug!("selected word: {}", word.id());
            imp.content.bind(&word);
            imp.content.set_fields_sensitive(true);
        } else {
            imp.content.clear_fields();
            imp.content.set_fields_sensitive(false);
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
            if let Some(word) = imp.sidebar.selected_word() {
                imp.sidebar.notify_changes_to_model(&word);
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
                    && imp.leaflet.visible_child_name() != Some("sidebar".into()),
            );
        }
    }
}

impl ui::View for DictionaryView {
    fn load_state(&self) {
        log::debug!("Loading view state.");

        let imp = self.imp();

        imp.sidebar.load_state();
        imp.content.load_state();

        self.load_selected_word();
    }

    fn unload_state(&self) {
        log::debug!("Unloading view state.");

        let imp = self.imp();
        imp.sidebar.unload_state();
        imp.content.unload_state();
    }

    fn connect_headerbar(&self, header_bar: &ui::HeaderBar) {
        let imp = self.imp();

        header_bar
            .imp()
            .back_button
            .connect_clicked(glib::clone!(@weak self as view => move |_| {
                view.activate_action("dictionary.go-back", None).unwrap_or_default();
            }));

        imp.header_bar.replace(Some(header_bar.clone()));
    }
}
