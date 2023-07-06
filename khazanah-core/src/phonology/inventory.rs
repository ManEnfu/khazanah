use std::collections::{hash_map::Keys, HashMap};

use bimap::BiHashMap;
use uuid::Uuid;

use crate::{utils, Phoneme};

/// An inventory of phonemes.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Inventory {
    phonemes: HashMap<Uuid, Phoneme>,

    romanization_pronunciation_map: Option<BiHashMap<String, String>>,
}

impl Inventory {
    /// Creates a new inventory.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a phoneme into the inventory and returns its id.
    pub fn add_phoneme(&mut self, mut phoneme: Phoneme) -> Uuid {
        let id = if let Some(id) = phoneme.id() {
            id
        } else {
            phoneme.generate_id()
        };
        self.phonemes.insert(id, phoneme);
        id
    }

    /// Removes a phoneme of id `id` from the inventory.
    pub fn delete_phoneme_by_id(&mut self, id: Uuid) -> Option<Phoneme> {
        self.phonemes.remove(&id)
    }

    /// Gets the number of phonemes.
    pub fn n_phonemes(&self) -> usize {
        self.phonemes.len()
    }

    /// Gets a reference to phoneme by id.
    pub fn phoneme_by_id(&self, id: Uuid) -> Option<&Phoneme> {
        self.phonemes.get(&id)
    }

    /// Gets a mutable reference to phoneme by id.
    pub fn phoneme_by_id_mut(&mut self, id: Uuid) -> Option<&mut Phoneme> {
        // Assume that the romanization map is outdated.
        self.romanization_pronunciation_map = None;
        self.phonemes.get_mut(&id)
    }

    /// Iterates over phonemes.
    pub fn iter_phonemes(&self) -> impl Iterator<Item = &Phoneme> {
        self.phonemes.values()
    }

    /// Iterates over mutable reference of phonemes.
    pub fn iter_phonemes_mut(&mut self) -> impl Iterator<Item = &mut Phoneme> {
        self.phonemes.values_mut()
    }

    /// Iterates over phoneme ids.
    pub fn ids(&self) -> Keys<Uuid, Phoneme> {
        self.phonemes.keys()
    }

    /// Converts a romanization to IPA pronunciation using rules specified by the inventory.
    pub fn pronunce_romanization(&mut self, romanization: &str) -> String {
        if self.romanization_pronunciation_map.is_none() {
            self.populate_romanization_pronunciation_map();
        }
        if let Some(map) = &self.romanization_pronunciation_map {
            String::from_iter(utils::transliterate(romanization, 5, |s| {
                map.get_by_left(s).cloned()
            }))
        } else {
            String::default()
        }
    }

    /// Converts an IPA pronunciation to romanization using rules specified by the inventory.
    pub fn get_romanization(&mut self, pronunciation: &str) -> String {
        if self.romanization_pronunciation_map.is_none() {
            self.populate_romanization_pronunciation_map();
        }
        if let Some(map) = &self.romanization_pronunciation_map {
            String::from_iter(utils::transliterate(pronunciation, 5, |s| {
                map.get_by_right(s).cloned()
            }))
        } else {
            String::default()
        }
    }

    fn populate_romanization_pronunciation_map(&mut self) {
        let mut map = BiHashMap::new();
        for phoneme in self.iter_phonemes() {
            map.insert(
                phoneme
                    .romanization()
                    .unwrap_or(phoneme.sound())
                    .to_string(),
                phoneme.sound().to_string(),
            );
        }
        self.romanization_pronunciation_map = Some(map);
    }
}

#[cfg(test)]
mod tests {
    use crate::phonology::PhonemeBuilder;

    use super::*;

    fn test_inventory() -> Inventory {
        let mut inv = Inventory::new();
        inv.add_phoneme(
            PhonemeBuilder::new()
                .sound("ɬ".to_string())
                .romanization("hl".to_string())
                .build(),
        );
        inv.add_phoneme(PhonemeBuilder::new().sound("t".to_string()).build());
        inv.add_phoneme(PhonemeBuilder::new().sound("h".to_string()).build());
        inv.add_phoneme(
            PhonemeBuilder::new()
                .sound("ɹ".to_string())
                .romanization("r".to_string())
                .build(),
        );
        inv.add_phoneme(PhonemeBuilder::new().sound("s".to_string()).build());
        inv.add_phoneme(PhonemeBuilder::new().sound("m".to_string()).build());
        inv.add_phoneme(
            PhonemeBuilder::new()
                .sound("tʰ".to_string())
                .romanization("th".to_string())
                .build(),
        );
        inv.add_phoneme(
            PhonemeBuilder::new()
                .sound("t͡s".to_string())
                .romanization("ts".to_string())
                .build(),
        );
        inv.add_phoneme(PhonemeBuilder::new().sound("a".to_string()).build());
        inv.add_phoneme(
            PhonemeBuilder::new()
                .sound("aː".to_string())
                .romanization("aa".to_string())
                .build(),
        );
        inv.add_phoneme(
            PhonemeBuilder::new()
                .sound("ɪ".to_string())
                .romanization("i".to_string())
                .build(),
        );
        inv.add_phoneme(PhonemeBuilder::new().sound("u".to_string()).build());
        inv.add_phoneme(PhonemeBuilder::new().sound("e".to_string()).build());
        inv.add_phoneme(PhonemeBuilder::new().sound("o".to_string()).build());
        inv.add_phoneme(
            PhonemeBuilder::new()
                .sound("ə".to_string())
                .romanization("ë".to_string())
                .build(),
        );
        inv
    }

    #[test]
    fn romanization_and_pronunciation() {
        let mut inv = test_inventory();
        let romanization = "thëmaartsim".to_string();
        let pronunciation = "ˈtʰə.maːɹ.t͡sɪm".to_string();
        let pronunciation_no_delimiter = "tʰəmaːɹt͡sɪm".to_string();
        let _a = inv.get_romanization(&pronunciation);
        dbg!(&inv);
        assert_eq!(inv.get_romanization(&pronunciation), romanization);
        assert_eq!(
            inv.pronunce_romanization(&romanization),
            pronunciation_no_delimiter
        );
    }
}
