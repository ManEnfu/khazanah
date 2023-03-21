use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gdk, glib};

use adw::subclass::prelude::*;

use crate::ui;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
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
        pub toast_overlay: TemplateChild<adw::ToastOverlay>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for IpaChartViewWindow {
        const NAME: &'static str = "KhzIpaChartViewWindow";
        type Type = super::IpaChartViewWindow;
        type ParentType = adw::Window;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for IpaChartViewWindow {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_charts();
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

    /// Copies selected symbol to clipboard.
    #[template_callback]
    fn handle_symbol_selected(&self, sym: String, _chart: &ui::IpaChart) {
        if let Some(clipboard) = gdk::Display::default().map(|d| d.clipboard()) {
            clipboard.set_text(&sym);
            let msg = format!("{} Copied", &sym);
            self.imp().toast_overlay.add_toast(adw::Toast::new(&msg));
        }
    }
}

impl Default for IpaChartViewWindow {
    fn default() -> Self {
        Self::new()
    }
}

impl ui::View for IpaChartViewWindow {}
