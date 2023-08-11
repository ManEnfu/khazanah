use std::collections::HashMap;

use uuid::Uuid;

use crate::xml::{ReadXml, WriteXml, XmlError, XmlReader, XmlWriter};

pub trait IdAble {
    fn id(&self) -> Option<Uuid>;

    fn generate_id(&mut self) -> Uuid;
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Store<T> {
    inner: HashMap<Uuid, T>,
}

impl<T> Store<T>
where
    T: IdAble + Default,
{
    pub fn new() -> Self {
        Self {
            inner: HashMap::default(),
        }
    }

    pub fn add(&mut self, mut v: T) -> Uuid {
        let id = if let Some(id) = v.id() {
            id
        } else {
            v.generate_id()
        };
        self.inner.insert(id, v);
        id
    }

    pub fn remove(&mut self, id: Uuid) -> Option<T> {
        self.inner.remove(&id)
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn get(&self, id: Uuid) -> Option<&T> {
        self.inner.get(&id)
    }

    pub fn get_mut(&mut self, id: Uuid) -> Option<&mut T> {
        self.inner.get_mut(&id)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.inner.values()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.inner.values_mut()
    }

    pub fn ids(&self) -> impl Iterator<Item = &Uuid> {
        self.inner.keys()
    }
}

impl<T> Store<T>
where
    T: IdAble + Default + ReadXml,
{
    pub(crate) fn _process_tag_start<R, S, E>(
        &mut self,
        root_tag: &str,
        reader: &mut XmlReader<R>,
        _state: &mut S,
        name: String,
        attrs: Vec<(String, String)>,
    ) -> Result<(), XmlError<E>>
    where
        R: std::io::BufRead,
        XmlError<E>: From<XmlError<<T as ReadXml>::Error>>,
    {
        let t_tag = <T as ReadXml>::TAG;
        match reader.last_tag_pair() {
            (_, Some(rt)) if rt == root_tag => {}
            (Some(rt), Some(tt)) if rt == root_tag && tt == t_tag => {
                let v = T::deserialize_xml(reader, Some((name, attrs)))?;
                self.add(v);
            }
            _ => {
                return Err(XmlError::InvalidTag(name));
            }
        }
        Ok(())
    }
}

impl<T> Store<T>
where
    T: IdAble + Default + WriteXml,
{
    pub(crate) fn _serialize_xml<W, E>(
        &self,
        root_tag: &str,
        writer: &mut XmlWriter<W>,
    ) -> Result<(), XmlError<E>>
    where
        W: std::io::Write,
        XmlError<E>: From<XmlError<<T as WriteXml>::Error>>,
    {
        writer.write_tag_start(root_tag)?;

        for (_, cat) in self.inner.iter() {
            cat.serialize_xml(writer)?;
        }

        writer.write_tag_end(root_tag)?;

        Ok(())
    }
}

impl<T> ReadXml for Store<T>
where
    T: IdAble + Default + ReadXml,
{
    type Error = T::Error;

    type ReaderState = ();

    const TAG: &'static str = "store";

    fn process_tag_start<R: std::io::BufRead>(
        &mut self,
        reader: &mut XmlReader<R>,
        state: &mut Self::ReaderState,
        name: String,
        attrs: Vec<(String, String)>,
    ) -> Result<(), XmlError<Self::Error>> {
        self._process_tag_start(Self::TAG, reader, state, name, attrs)
    }

    fn process_text<R: std::io::BufRead>(
        &mut self,
        _reader: &mut XmlReader<R>,
        _state: &mut Self::ReaderState,
        _text: String,
    ) -> Result<(), XmlError<Self::Error>> {
        Ok(())
    }

    fn process_tag_end<R: std::io::BufRead>(
        &mut self,
        _reader: &mut XmlReader<R>,
        _state: &mut Self::ReaderState,
        _name: String,
    ) -> Result<(), XmlError<Self::Error>> {
        Ok(())
    }
}

impl<T> WriteXml for Store<T>
where
    T: IdAble + Default + WriteXml,
{
    type Error = T::Error;

    fn serialize_xml<W: std::io::Write>(
        &self,
        writer: &mut XmlWriter<W>,
    ) -> Result<(), XmlError<Self::Error>> {
        self._serialize_xml("store", writer)
    }
}

#[cfg(test)]
mod tests {
    use uuid::uuid;

    use super::*;

    #[derive(Debug, Default, PartialEq, Eq)]
    struct Item {
        pub id: Option<Uuid>,
        pub value: String,
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

    impl ReadXml for Item {
        type Error = ();

        type ReaderState = ();

        const TAG: &'static str = "item";

        fn process_tag_start<R: std::io::BufRead>(
            &mut self,
            reader: &mut XmlReader<R>,
            _state: &mut Self::ReaderState,
            name: String,
            attrs: Vec<(String, String)>,
        ) -> Result<(), XmlError<Self::Error>> {
            match reader.last_tag_pair() {
                (_, Some(Self::TAG)) => {
                    let id = attrs
                        .iter()
                        .find(|&x| x.0 == "id")
                        .map(|x| Uuid::parse_str(&x.1))
                        .unwrap_or_else(|| Ok(Uuid::new_v4()))
                        .map_err(|_| XmlError::Other(()))?;
                    self.id = Some(id);
                }
                (Some(Self::TAG), Some("value")) => {
                    self.value.clear();
                }
                _ => return Err(XmlError::InvalidTag(name)),
            }

            Ok(())
        }

        fn process_text<R: std::io::BufRead>(
            &mut self,
            reader: &mut XmlReader<R>,
            _state: &mut Self::ReaderState,
            text: String,
        ) -> Result<(), XmlError<Self::Error>> {
            match reader.last_tag() {
                Some("value") => {
                    self.value += &text;
                }
                _ => {}
            }

            Ok(())
        }

        fn process_tag_end<R: std::io::BufRead>(
            &mut self,
            _reader: &mut XmlReader<R>,
            _state: &mut Self::ReaderState,
            _name: String,
        ) -> Result<(), XmlError<Self::Error>> {
            Ok(())
        }
    }

    impl WriteXml for Item {
        type Error = ();

        fn serialize_xml<W: std::io::Write>(
            &self,
            writer: &mut XmlWriter<W>,
        ) -> Result<(), XmlError<Self::Error>> {
            if let Some(id) = self.id {
                writer
                    .write_tag_start_with_attributes("item", [("id", id.to_string().as_str())])?;
            } else {
                writer.write_tag_start("item")?;
            };

            writer.write_tag_start("value")?;
            writer.write_text(&self.value)?;
            writer.write_tag_end("value")?;

            writer.write_tag_end("item")?;

            Ok(())
        }
    }

    const XML1: &str = r#"
    <store>
        <item id="74a61b73-2830-4d23-80d7-fe3222741e80">
            <value>the</value>
        </item>
        <item id="bc629cc3-99be-44f0-a2c4-d51dce960f2c">
            <value>north</value>
        </item>
        <item id="07d1d0ce-3f14-4708-bcda-14b2413fbe8e">
            <value>wind</value>
        </item>
        <item id="ae835d0b-b4ce-4686-b16f-d7fbbec55d96">
            <value>and</value>
        </item>
        <item id="fdd685d9-9a96-42b0-856c-fd3b7de584e7">
            <value>the</value>
        </item>
        <item id="5ce2f1b7-527f-4779-9c96-71939cb397af">
            <value>sun</value>
        </item>
    </store>
    "#;

    #[test]
    fn read_xml() {
        let store = Store::<Item>::load_xml_str(XML1).unwrap();

        let item = store
            .get(uuid!("74a61b73-2830-4d23-80d7-fe3222741e80"))
            .unwrap();
        assert_eq!(&item.value, "the");

        let item = store
            .get(uuid!("fdd685d9-9a96-42b0-856c-fd3b7de584e7"))
            .unwrap();
        assert_eq!(&item.value, "the");

        let item = store
            .get(uuid!("bc629cc3-99be-44f0-a2c4-d51dce960f2c"))
            .unwrap();
        assert_eq!(&item.value, "north");

        let item = store
            .get(uuid!("07d1d0ce-3f14-4708-bcda-14b2413fbe8e"))
            .unwrap();
        assert_eq!(&item.value, "wind");

        let item = store
            .get(uuid!("5ce2f1b7-527f-4779-9c96-71939cb397af"))
            .unwrap();
        assert_eq!(&item.value, "sun");

        let item = store
            .get(uuid!("ae835d0b-b4ce-4686-b16f-d7fbbec55d96"))
            .unwrap();
        assert_eq!(&item.value, "and");
    }

    #[test]
    fn write_xml() {
        let store = Store::<Item>::load_xml_str(XML1).unwrap();
        let xml2 = store.save_xml_string().unwrap();
        let store2 = Store::<Item>::load_xml_str(&xml2).unwrap();
        assert_eq!(&store, &store2);
    }
}
