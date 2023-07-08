use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gdk, gio, glib};

use adw::subclass::prelude::*;
use khazanah_core::{Ipa, Phoneme};

use crate::{models, ui};
use list_row::ListRow;

const EXPECTED_LIST_ITEM: &str = "Expected object to be `GtkListItem`";
const EXPECTED_LIST_ROW: &str = "Expected object to be `KhzIpaChartViewWindowListRow`";
const EXPECTED_PHONEME_OBJECT: &str = "Expected object to be `KhzPhonemeObject`";

mod list_row;

#[doc(hidden)]
#[allow(clippy::enum_variant_names)]
mod imp {
    use std::cell::RefCell;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type = super::IpaChartViewWindow)]
    #[template(resource = "/com/github/manenfu/Khazanah/ui/ipa_chart_view_window.ui")]
    pub struct IpaChartViewWindow {
        #[template_child]
        pub pulmonic_consonants_chart: TemplateChild<ui::IpaChart>,
        #[template_child]
        pub coarticulated_consonants_chart: TemplateChild<ui::IpaChart>,
        #[template_child]
        pub ejective_consonants_chart: TemplateChild<ui::IpaChart>,
        #[template_child]
        pub implosive_consonants_chart: TemplateChild<ui::IpaChart>,
        #[template_child]
        pub click_consonants_chart: TemplateChild<ui::IpaChart>,
        #[template_child]
        pub vowels_chart: TemplateChild<ui::IpaChart>,

        #[template_child]
        pub search_entry: TemplateChild<gtk::SearchEntry>,
        #[template_child]
        pub list_view: TemplateChild<gtk::ListView>,

        #[template_child]
        pub stack: TemplateChild<gtk::Stack>,

        #[template_child]
        pub toast_overlay: TemplateChild<adw::ToastOverlay>,

        #[property(get, set)]
        pub list_model: RefCell<Option<gio::ListStore>>,
        #[property(get, set)]
        pub filter_model: RefCell<Option<gtk::FilterListModel>>,
        #[property(get, set)]
        pub selection_model: RefCell<Option<gtk::NoSelection>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for IpaChartViewWindow {
        const NAME: &'static str = "KhzIpaChartViewWindow";
        type Type = super::IpaChartViewWindow;
        type ParentType = adw::Window;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
            klass.install_action("win.search", None, |win, _, _| {
                win.imp().search_entry.grab_focus();
            });
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for IpaChartViewWindow {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            self.search_entry.set_key_capture_widget(Some(&*obj));

            obj.setup_charts();
            obj.setup_list();
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

    impl WidgetImpl for IpaChartViewWindow {}

    impl WindowImpl for IpaChartViewWindow {}
    impl AdwWindowImpl for IpaChartViewWindow {}
}

glib::wrapper! {
    /// X-SAMPA transliteration tool as a window.
    pub struct IpaChartViewWindow(ObjectSubclass<imp::IpaChartViewWindow>)
        @extends gtk::Widget, gtk::Window, adw::Window;
}

#[gtk::template_callbacks]
impl IpaChartViewWindow {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    /// Setups and populates IPA charts.
    pub fn setup_charts(&self) {
        let imp = self.imp();
        imp.pulmonic_consonants_chart.populate_pulmonic_consonants();
        imp.coarticulated_consonants_chart
            .populate_coarticulated_consonants();
        imp.ejective_consonants_chart.populate_ejective_consonants();
        imp.implosive_consonants_chart
            .populate_implosive_consonants();
        imp.click_consonants_chart.populate_click_consonants();
        imp.vowels_chart.populate_vowels();
    }

    pub fn setup_list(&self) {
        let imp = self.imp();

        let mut list_model = gio::ListStore::new(models::PhonemeObject::static_type());
        list_model.extend(
            Ipa::all_valids().map(|i| models::PhonemeObject::from_phoneme(Phoneme::from_ipa(i))),
        );

        self.set_list_model(list_model.clone());

        let filter_model =
            gtk::FilterListModel::new(Some(list_model), Some(gtk::CustomFilter::new(|_| true)));
        self.set_filter_model(filter_model.clone());

        let selection_model = gtk::NoSelection::new(Some(filter_model));
        self.set_selection_model(selection_model.clone());

        let factory = gtk::SignalListItemFactory::new();

        factory.connect_setup(glib::clone!(@weak self as view => move |_, item| {
            let row = ListRow::new();

            item.downcast_ref::<gtk::ListItem>()
                .expect(EXPECTED_LIST_ITEM)
                .set_child(Some(&row));
        }));

        factory.connect_bind(move |_, item| {
            let list_item = item
                .downcast_ref::<gtk::ListItem>()
                .expect(EXPECTED_LIST_ITEM);

            let word_object = list_item
                .item()
                .and_downcast::<models::PhonemeObject>()
                .expect(EXPECTED_PHONEME_OBJECT);

            let row = list_item
                .child()
                .and_downcast::<ListRow>()
                .expect(EXPECTED_LIST_ROW);

            row.bind(&word_object);
        });

        factory.connect_unbind(move |_, item| {
            let list_item = item
                .downcast_ref::<gtk::ListItem>()
                .expect(EXPECTED_LIST_ITEM);

            let row = list_item
                .child()
                .and_downcast::<ListRow>()
                .expect(EXPECTED_LIST_ROW);

            row.unbind();
        });

        imp.list_view.set_model(Some(&selection_model));
        imp.list_view.set_factory(Some(&factory));
    }

    pub fn search(&self, text: Option<String>) {
        let imp = self.imp();

        if let Some(text) = text {
            if let Some(filter_model) = self.filter_model() {
                if let Some(filter) = filter_model
                    .filter()
                    .and_then(|f| f.downcast::<gtk::CustomFilter>().ok())
                {
                    filter.set_filter_func(move |p| {
                        p.downcast_ref::<models::PhonemeObject>()
                            .map(|p| {
                                p.name().to_lowercase().contains(&text)
                                    || p.sound().to_lowercase().contains(&text)
                            })
                            .unwrap_or_default()
                    })
                }
                if filter_model.n_items() > 0 {
                    imp.stack.set_visible_child_name("list");
                } else {
                    imp.stack.set_visible_child_name("empty");
                }
            }
        } else {
            imp.stack.set_visible_child_name("charts");
        }
    }

    /// Copies selected symbol to clipboard.
    #[template_callback]
    fn handle_symbol_selected(&self, sym: String, _chart: &ui::IpaChart) {
        if let Some(clipboard) = gdk::Display::default().map(|d| d.clipboard()) {
            clipboard.set_text(&sym);
            let msg = format!("{} Copied", &sym);
            self.imp().toast_overlay.add_toast(adw::Toast::new(&msg));
        }
    }

    #[template_callback]
    fn handle_list_row_activated(&self, position: u32, list_view: &gtk::ListView) {
        if let Some(p) = list_view
            .model()
            .and_then(|m| m.item(position))
            .and_downcast::<models::PhonemeObject>()
        {
            let sym = p.sound();
            if let Some(clipboard) = gdk::Display::default().map(|d| d.clipboard()) {
                clipboard.set_text(&sym);
                let msg = format!("{} Copied", &sym);
                self.imp().toast_overlay.add_toast(adw::Toast::new(&msg));
            }
        }
    }

    #[template_callback]
    fn handle_search_entry_started(&self, entry: &gtk::SearchEntry) {
        entry.grab_focus();
    }

    #[template_callback]
    fn handle_search_entry_changed(&self, entry: &gtk::SearchEntry) {
        let text = entry.text().to_ascii_lowercase();
        if text.is_empty() {
            self.search(None);
        } else {
            self.search(Some(text));
        }
    }

    #[template_callback]
    fn handle_search_entry_stopped(&self, entry: &gtk::SearchEntry) {
        entry.set_text("");
    }
}

impl Default for IpaChartViewWindow {
    fn default() -> Self {
        Self::new()
    }
}

impl ui::View for IpaChartViewWindow {}
