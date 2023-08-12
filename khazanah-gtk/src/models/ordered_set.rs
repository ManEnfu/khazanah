use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

use khazanah_core::prelude::*;
use khazanah_core::Store;
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
    #[properties(wrapper_type = super::KeyStore)]
    pub struct KeyStore {
        #[property(get)]
        pub inner: RefCell<gio::ListStore>,

        pub key_orders: RefCell<HashMap<Uuid, KeyOrder>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for KeyStore {
        const NAME: &'static str = "KhzOrderedSet";
        type Type = super::KeyStore;
        type Interfaces = (gio::ListModel,);
    }

    impl ObjectImpl for KeyStore {
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
            let obj = self.obj();

            self.inner.borrow().connect_items_changed(
                glib::clone!(@strong obj as set => move |_, position, removed, added| {
                    set.items_changed(position, removed, added);
                }),
            );
        }
    }

    impl ListModelImpl for KeyStore {
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
    pub struct KeyStore(ObjectSubclass<imp::KeyStore>)
        @implements gio::ListModel;
}

impl KeyStore {
    pub fn new(_item_type: glib::Type) -> Self {
        glib::Object::builder().build()
    }

    pub fn insert(&self, key: Uuid, item: &impl IsA<glib::Object>) {
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

    pub fn contains_id(&self, key: &Uuid) -> bool {
        let imp = self.imp();
        imp.key_orders.borrow().contains_key(key)
    }

    pub fn remove_by_id(&self, key: &Uuid) {
        let imp = self.imp();

        let mut kos = imp.key_orders.borrow_mut();
        if let Some(ko) = kos.remove(key) {
            let n = ko.order;
            for iko in kos.values_mut() {
                if iko.order > n {
                    iko.order -= 1;
                }
            }

            imp.inner.borrow().remove(ko.order);
        }
    }

    pub fn remove_all(&self) {
        let imp = self.imp();
        imp.key_orders.borrow_mut().clear();
        imp.inner.borrow().remove_all();
    }

    pub fn updated_by_id(&self, key: &Uuid) {
        let imp = self.imp();

        if let Some(ko) = imp.key_orders.borrow().get(key) {
            self.items_changed(ko.order, 1, 1);
        }
    }

    pub fn sync_with_store<T, F, O>(&self, store: &Store<T>, factory: F)
    where
        T: IdAble + Default,
        F: Fn(Uuid, &T) -> O,
        O: IsA<glib::Object>,
    {
        let imp = self.imp();

        // iterates `self`, removes items not in `store`
        let mut to_be_removed = Vec::<Uuid>::new();

        for key in imp.key_orders.borrow().keys() {
            if !store.contains(*key) {
                to_be_removed.push(*key);
            }
        }

        for key in to_be_removed {
            self.remove_by_id(&key);
        }

        // iterates `store`, adds items in `store` but not in `self`
        for item in store.iter() {
            if let Some(id) = item.id() {
                if !self.contains_id(&id) {
                    let obj = factory(id, item);
                    self.insert(id, obj.upcast_ref::<glib::Object>());
                }
            }
        }
    }
}

impl<A> Extend<(Uuid, A)> for KeyStore
where
    A: IsA<glib::Object>,
{
    fn extend<T: IntoIterator<Item = (Uuid, A)>>(&mut self, iter: T) {
        for (key, item) in iter.into_iter() {
            self.insert(key, item.upcast_ref::<glib::Object>());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Default, PartialEq, Eq)]
    struct Item {
        pub id: Option<Uuid>,
    }

    impl IdAble for Item {
        fn id(&self) -> Option<Uuid> {
            self.id
        }

        fn generate_id(&mut self) -> Uuid {
            let id = Uuid::new_v4();
            self.id = Some(id);
            id
        }
    }

    #[gtk::test]
    fn sync() {
        let mut store = Store::<Item>::new();
        let os = KeyStore::new(gtk::Label::static_type());

        let id1 = store.add(Item::default());
        let id2 = store.add(Item::default());
        let id3 = store.add(Item::default());
        let id4 = store.add(Item::default());
        let id5 = store.add(Item::default());
        let id6 = store.add(Item::default());

        os.sync_with_store(&store, |i, _| gtk::Label::new(Some(i.to_string().as_str())));

        assert!(os.contains_id(&id1));
        assert_eq!(
            os.get_by_id(&id1)
                .and_downcast::<gtk::Label>()
                .unwrap()
                .label(),
            id1.to_string().as_str()
        );
        assert!(os.contains_id(&id2));
        assert_eq!(
            os.get_by_id(&id2)
                .and_downcast::<gtk::Label>()
                .unwrap()
                .label(),
            id2.to_string().as_str()
        );
        assert!(os.contains_id(&id3));
        assert_eq!(
            os.get_by_id(&id3)
                .and_downcast::<gtk::Label>()
                .unwrap()
                .label(),
            id3.to_string().as_str()
        );
        assert!(os.contains_id(&id4));
        assert_eq!(
            os.get_by_id(&id4)
                .and_downcast::<gtk::Label>()
                .unwrap()
                .label(),
            id4.to_string().as_str()
        );
        assert!(os.contains_id(&id5));
        assert_eq!(
            os.get_by_id(&id5)
                .and_downcast::<gtk::Label>()
                .unwrap()
                .label(),
            id5.to_string().as_str()
        );
        assert!(os.contains_id(&id6));
        assert_eq!(
            os.get_by_id(&id6)
                .and_downcast::<gtk::Label>()
                .unwrap()
                .label(),
            id6.to_string().as_str()
        );

        store.remove(id2);
        store.remove(id5);
        let id7 = store.add(Item::default());

        os.sync_with_store(&store, |i, _| gtk::Label::new(Some(i.to_string().as_str())));

        assert!(os.contains_id(&id1));
        assert_eq!(
            os.get_by_id(&id1)
                .and_downcast::<gtk::Label>()
                .unwrap()
                .label(),
            id1.to_string().as_str()
        );
        assert!(!os.contains_id(&id2));
        assert!(os.contains_id(&id3));
        assert_eq!(
            os.get_by_id(&id3)
                .and_downcast::<gtk::Label>()
                .unwrap()
                .label(),
            id3.to_string().as_str()
        );
        assert!(os.contains_id(&id4));
        assert_eq!(
            os.get_by_id(&id4)
                .and_downcast::<gtk::Label>()
                .unwrap()
                .label(),
            id4.to_string().as_str()
        );
        assert!(!os.contains_id(&id5));
        assert!(os.contains_id(&id6));
        assert_eq!(
            os.get_by_id(&id6)
                .and_downcast::<gtk::Label>()
                .unwrap()
                .label(),
            id6.to_string().as_str()
        );
        assert!(os.contains_id(&id7));
        assert_eq!(
            os.get_by_id(&id7)
                .and_downcast::<gtk::Label>()
                .unwrap()
                .label(),
            id7.to_string().as_str()
        );
    }
}
