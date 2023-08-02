use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

use uuid::Uuid;

#[derive(Debug)]
pub struct KeyOrder {
    pub order: u32,
    pub _reserved: bool,
}

#[doc(hidden)]
mod imp {
    use std::{cell::RefCell, collections::HashMap};

    use super::*;

    #[derive(Debug, Default, glib::Properties)]
    #[properties(wrapper_type = super::OrderedSet)]
    pub struct OrderedSet {
        #[property(get, set)]
        pub inner: RefCell<gio::ListStore>,

        pub key_orders: RefCell<HashMap<Uuid, KeyOrder>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for OrderedSet {
        const NAME: &'static str = "KhzOrderedSet";
        type Type = super::OrderedSet;
        type Interfaces = (gio::ListModel,);
    }

    impl ObjectImpl for OrderedSet {
        fn properties() -> &'static [glib::ParamSpec] {
            Self::derived_properties()
        }

        fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            self.derived_set_property(id, value, pspec)
        }

        fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            self.derived_property(id, pspec)
        }

        fn constructed(&self) {
            self.parent_constructed();

            self.inner.borrow().connect_items_changed(
                glib::clone!(@weak self as imp => move |_, position, removed, added| {
                    imp.obj().items_changed(position, removed, added);
                }),
            );
        }
    }

    impl ListModelImpl for OrderedSet {
        fn item_type(&self) -> glib::Type {
            self.inner.borrow().item_type()
        }

        fn n_items(&self) -> u32 {
            self.inner.borrow().n_items()
        }

        fn item(&self, position: u32) -> Option<glib::Object> {
            self.inner.borrow().item(position)
        }
    }
}

glib::wrapper! {
    /// A set with `Uuid` as key represented by an ordered list.
    pub struct OrderedSet(ObjectSubclass<imp::OrderedSet>)
        @implements gio::ListModel;
}

impl OrderedSet {
    pub fn new(item_type: glib::Type) -> Self {
        glib::Object::builder()
            .property("inner", gio::ListStore::new(item_type))
            .build()
    }

    pub fn insert(&self, key: Uuid, item: &glib::Object) {
        let imp = self.imp();

        let n = imp.inner.borrow().n_items();
        imp.key_orders.borrow_mut().insert(
            key,
            KeyOrder {
                order: n,
                _reserved: false,
            },
        );
        imp.inner.borrow().append(item);
    }

    pub fn get_by_id(&self, key: &Uuid) -> Option<glib::Object> {
        let imp = self.imp();

        if let Some(ko) = imp.key_orders.borrow().get(key) {
            imp.inner.borrow().item(ko.order)
        } else {
            None
        }
    }

    pub fn remove_by_id(&self, key: &Uuid) {
        let imp = self.imp();

        if let Some(ko) = imp.key_orders.borrow_mut().remove(key) {
            imp.inner.borrow().remove(ko.order);
        }
    }

    pub fn remove_all(&self) {
        let imp = self.imp();
        imp.key_orders.borrow_mut().clear();
        imp.inner.borrow().remove_all();
    }
}

impl<A> Extend<(Uuid, A)> for OrderedSet
where
    A: IsA<glib::Object>,
{
    fn extend<T: IntoIterator<Item = (Uuid, A)>>(&mut self, iter: T) {
        for (key, item) in iter.into_iter() {
            self.insert(key, item.upcast_ref::<glib::Object>());
        }
    }
}
