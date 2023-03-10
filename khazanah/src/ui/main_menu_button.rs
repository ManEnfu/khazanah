use gtk::glib;
use gtk::subclass::prelude::*;

use adw::subclass::prelude::*;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/manenfu/Khazanah/ui/main_menu_button.ui")]
    pub struct MainMenuButton {
        #[template_child]
        pub main_menu_button: TemplateChild<gtk::MenuButton>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MainMenuButton {
        const NAME: &'static str = "KhzMainMenuButton";
        type Type = super::MainMenuButton;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for MainMenuButton {}

    impl WidgetImpl for MainMenuButton {}
    impl BinImpl for MainMenuButton {}
}

glib::wrapper! {
    /// Main menu button.
    pub struct MainMenuButton(ObjectSubclass<imp::MainMenuButton>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}
