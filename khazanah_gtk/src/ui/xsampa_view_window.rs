use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gdk, glib};

use adw::subclass::prelude::*;

use khazanah_core::ipa;

use crate::ui;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/manenfu/Khazanah/ui/xsampa_view_window.ui")]
    pub struct XSampaViewWindow {
        #[template_child]
        pub xsampa_entry: TemplateChild<gtk::Entry>,

        #[template_child]
        pub ipa_entry: TemplateChild<gtk::Entry>,

        #[template_child]
        pub toast_overlay: TemplateChild<adw::ToastOverlay>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for XSampaViewWindow {
        const NAME: &'static str = "KhzXSampaViewWindow";
        type Type = super::XSampaViewWindow;
        type ParentType = adw::Window;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();

            klass.install_action("xsampa.copy-ipa", None, move |widget, _, _| {
                let text = widget.imp().ipa_entry.text();

                if let Some(clipboard) = gdk::Display::default().map(|d| d.clipboard()) {
                    clipboard.set_text(&text);
                    widget
                        .imp()
                        .toast_overlay
                        .add_toast(adw::Toast::new("Copied"));
                }
            });
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for XSampaViewWindow {
        fn constructed(&self) {
            self.parent_constructed();
            self.obj().setup_bindings();
        }
    }

    impl WidgetImpl for XSampaViewWindow {}

    impl WindowImpl for XSampaViewWindow {}
    impl AdwWindowImpl for XSampaViewWindow {}
}

glib::wrapper! {
    /// X-SAMPA transliteration tool as a window.
    pub struct XSampaViewWindow(ObjectSubclass<imp::XSampaViewWindow>)
        @extends gtk::Widget, gtk::Window, adw::Window;
}

#[gtk::template_callbacks]
impl XSampaViewWindow {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn setup_bindings(&self) {
        let imp = self.imp();
        imp.xsampa_entry
            .bind_property("text", &imp.ipa_entry.get(), "text")
            .sync_create()
            .transform_to(|_, s: Option<String>| s.map(|s| ipa::transliterate_xsampa(&s)))
            .build();
    }

    #[template_callback]
    fn handle_icon_clicked(&self, icon: gtk::EntryIconPosition, entry: &gtk::Entry) {
        if icon == gtk::EntryIconPosition::Secondary {
            let text = entry.text();

            if let Some(clipboard) = gdk::Display::default().map(|d| d.clipboard()) {
                clipboard.set_text(&text);
                self.imp()
                    .toast_overlay
                    .add_toast(adw::Toast::new("Copied"));
            }
        }
    }
}

impl Default for XSampaViewWindow {
    fn default() -> Self {
        Self::new()
    }
}

impl ui::View for XSampaViewWindow {}
