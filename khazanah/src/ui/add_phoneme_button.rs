use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use adw::subclass::prelude::*;

use crate::models;

const EXPECTED_LIST_ITEM: &str = "Expected object to be `GtkListItem`";
const EXPECTED_PHONEME_OBJECT: &str = "Expected object to be `KhzPhonemeObject`";
const EXPECTED_ADD_PHONEME_LIST_ROW: &str = "Expected object to be `KhzAddPhonemeListRow`";

#[allow(clippy::enum_variant_names)]
mod imp {
    use std::cell::RefCell;

    use gtk::glib::subclass::Signal;
    use once_cell::sync::Lazy;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type = super::AddPhonemeButton)]
    #[template(resource = "/com/github/manenfu/Khazanah/ui/add_phoneme_button.ui")]
    pub struct AddPhonemeButton {
        #[template_child]
        pub button: TemplateChild<gtk::MenuButton>,
        #[template_child]
        pub popover: TemplateChild<gtk::Popover>,
        #[template_child]
        pub list_view: TemplateChild<gtk::ListView>,
        #[template_child]
        pub search_entry: TemplateChild<gtk::SearchEntry>,

        #[property(get, set)]
        pub phoneme_list_model: RefCell<Option<models::AddPhonemeListModel>>,
        #[property(get, set)]
        pub filter_model: RefCell<Option<gtk::FilterListModel>>,
        #[property(get, set)]
        pub selection_model: RefCell<Option<gtk::NoSelection>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AddPhonemeButton {
        const NAME: &'static str = "KhzAddPhonemeButton";
        type Type = super::AddPhonemeButton;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for AddPhonemeButton {
        fn constructed(&self) {
            let obj = self.obj();

            self.parent_constructed();
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

        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
                vec![Signal::builder("phoneme-selected")
                    .param_types([models::AddPhonemeObject::static_type()])
                    .build()]
            });
            SIGNALS.as_ref()
        }
    }

    impl WidgetImpl for AddPhonemeButton {}
    impl BinImpl for AddPhonemeButton {}
}

glib::wrapper! {
    /// Button to add phonemes.
    pub struct AddPhonemeButton(ObjectSubclass<imp::AddPhonemeButton>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

#[gtk::template_callbacks]
impl AddPhonemeButton {
    fn setup_list(&self) {
        let imp = self.imp();

        let phoneme_list_model = models::AddPhonemeListModel::new();
        self.set_phoneme_list_model(phoneme_list_model.clone());

        let filter_model = gtk::FilterListModel::new(
            Some(phoneme_list_model),
            Some(gtk::CustomFilter::new(|_| true)),
        );
        self.set_filter_model(filter_model.clone());

        let selection_model = gtk::NoSelection::new(Some(filter_model));
        self.set_selection_model(selection_model.clone());

        let factory = gtk::SignalListItemFactory::new();

        factory.connect_setup(glib::clone!(@weak self as view => move |_, item| {
            let row = super::AddPhonemeRow::new();

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
                .and_downcast::<models::AddPhonemeObject>()
                .expect(EXPECTED_PHONEME_OBJECT);

            let row = list_item
                .child()
                .and_downcast::<super::AddPhonemeRow>()
                .expect(EXPECTED_ADD_PHONEME_LIST_ROW);

            row.bind(&word_object);
        });

        factory.connect_unbind(move |_, item| {
            let list_item = item
                .downcast_ref::<gtk::ListItem>()
                .expect(EXPECTED_LIST_ITEM);

            let row = list_item
                .child()
                .and_downcast::<super::AddPhonemeRow>()
                .expect(EXPECTED_ADD_PHONEME_LIST_ROW);

            row.unbind();
        });

        imp.list_view.set_model(Some(&selection_model));
        imp.list_view.set_factory(Some(&factory));
    }

    #[template_callback]
    pub fn handle_search_entry_changed(&self, entry: &gtk::SearchEntry) {
        let text = entry.text().to_lowercase();

        if let Some(filter_model) = self.filter_model() {
            if let Some(filter) = filter_model
                .filter()
                .and_then(|f| f.downcast::<gtk::CustomFilter>().ok())
            {
                filter.set_filter_func(move |p| {
                    p.downcast_ref::<models::AddPhonemeObject>()
                        .map(|p| p.name().to_lowercase().contains(&text))
                        .unwrap_or_default()
                })
            }
        }
    }

    #[template_callback]
    pub fn handle_row_activated(&self, position: u32, list_view: &gtk::ListView) {
        if let Some(p) = list_view
            .model()
            .and_then(|m| m.item(position))
            .and_downcast::<models::AddPhonemeObject>()
        {
            self.emit_by_name::<()>("phoneme-selected", &[&p]);
            // log::debug!("Add phoneme: {}", p.name());
            self.imp().popover.popdown();
        }
    }
}
