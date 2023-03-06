use gtk::subclass::prelude::*;
use gtk::glib;

use adw::subclass::prelude::*;

use crate::ui;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/manenfu/Khazanah/ui/start_view.ui")]
    pub struct StartView {
        #[template_child]
        pub start_controls: TemplateChild<ui::ToolbarStartControls>,
        #[template_child]
        pub end_controls: TemplateChild<ui::ToolbarEndControls>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for StartView {
        const NAME: &'static str = "KhzStartView";
        type Type = super::StartView;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for StartView {}
    impl WidgetImpl for StartView {}
    impl BinImpl for StartView {}
}

glib::wrapper! {
    /// The first view when the application is started.
    pub struct StartView(ObjectSubclass<imp::StartView>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}
