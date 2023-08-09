use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gdk, glib};

use crate::models;
use crate::ui;

pub use content::Content;
pub use sidebar::Sidebar;

mod content;
mod phoneme_list_row;
mod sidebar;

#[doc(hidden)]
#[allow(clippy::enum_variant_names)]
mod imp {
    use std::cell::RefCell;

    use gtk::glib::subclass::Signal;
    use once_cell::sync::Lazy;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type = super::InventoryView)]
    #[template(resource = "/com/github/manenfu/Khazanah/ui/view/inventory.ui")]
    pub struct InventoryView {
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
    impl ObjectSubclass for InventoryView {
        const NAME: &'static str = "KhzInventoryView";
        type Type = super::InventoryView;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();

            klass.install_action("inventory.go-back", None, move |view, _, _| {
                view.navigate_backward();
            });

            klass.add_binding_action(
                gdk::Key::Escape,
                gdk::ModifierType::empty(),
                "inventory.go-back",
                None,
            );
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for InventoryView {
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

    impl WidgetImpl for InventoryView {}
    impl BinImpl for InventoryView {}
}

glib::wrapper! {
    pub struct InventoryView(ObjectSubclass<imp::InventoryView>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

#[gtk::template_callbacks]
impl InventoryView {
    #[template_callback]
    fn handle_sidebar_phoneme_selected(&self, _sidebar: &Sidebar) {
        self.load_selected_phoneme();
    }

    #[template_callback]
    fn handle_sidebar_search_changed(&self, _sidebar: &Sidebar) {
        self.load_selected_phoneme();
    }

    /// Loads form contents with word data.
    fn load_selected_phoneme(&self) {
        let imp = self.imp();
        let phoneme = imp.sidebar.selected_phoneme();
        imp.content.select_phoneme(phoneme.as_ref());

        self.navigate_backward();
    }

    #[template_callback]
    fn handle_sidebar_phoneme_activated(&self, _sidebar: &Sidebar) {
        let imp = self.imp();

        if imp.leaflet.is_folded() {
            self.navigate_forward();
        }
    }

    fn navigate_forward(&self) {
        let imp = self.imp();
        imp.leaflet.navigate(adw::NavigationDirection::Forward);
        self.update_buttons_visibility();
    }

    fn navigate_backward(&self) {
        let imp = self.imp();
        imp.leaflet.navigate(adw::NavigationDirection::Back);
        if imp.leaflet.is_folded() {
            if let Some(phoneme) = imp.sidebar.selected_phoneme() {
                imp.sidebar.phoneme_updated(&phoneme);
            }
        }
        self.update_buttons_visibility();
    }

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

impl ui::View for InventoryView {
    fn load_state(&self) {
        log::debug!("Loading view state.");

        let imp = self.imp();

        imp.sidebar.load_state();
        imp.content.load_state();

        self.load_selected_phoneme();
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
                view.activate_action("inventory.go-back", None).unwrap_or_default();
            }));

        imp.header_bar.replace(Some(header_bar.clone()));
    }

    fn go_back(&self) {
        self.navigate_backward();
    }
}
