#![allow(unused)]
use crate::prelude::*;

/// A map that is designed around quick lookups and vends KeyId instead of hashing your type.
/// - Inserting an element is amortized constant time
/// - Removing an element is not allowed
/// - Values are expected to be constructed with `Default::default()` and modified
///     - e.g. mapping a `&str` to `Vec<&str>` to model a graph. "remove" an element by clearing its vector.
/// - Values are lazily created
/// - Finding a KeyId from a Key is linear time
#[derive(Clone, Default)]
pub struct LookupMap<Key, Value> {
    keys: Vec<(KeyId, Key)>,
    entries: Vec<Entry<Key, Value>>,
}

#[derive(Copy, Debug, Clone, Default, PartialEq, Eq)]
pub struct KeyId(pub usize);

impl Display for KeyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "KeyId({})", self.0)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Entry<Key, Value> {
    // don't change this field
    pub id: KeyId,

    // don't change this field
    pub key: Key,

    // Feel free to change this field
    pub value: Value,
}

impl<Key, Value> LookupMap<Key, Value>
where
    Key: PartialEq + Ord,
    Value: Default,
{
    // "inherent associated types are unstable"
    // pub type Entry = Entry<Key, Value>;

    pub fn new() -> Self {
        Self {
            keys: vec![],
            entries: vec![],
        }
    }

    pub fn ids(&self) -> impl Iterator<Item = KeyId> + '_ {
        self.entries().map(|e| e.id)
    }

    pub fn entries(&self) -> impl Iterator<Item = &Entry<Key, Value>> {
        self.entries.iter()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Key, &Value)> {
        self.entries().map(|e| (&e.key, &e.value))
    }

    pub fn len(&self) -> usize {
        self.entries().count()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn key(&self, id: KeyId) -> &Key {
        &self.entries[id.0].key
    }

    pub fn value(&self, id: KeyId) -> &Value {
        &self.entries[id.0].value
    }

    pub fn entry(&mut self, id: KeyId) -> &mut Entry<Key, Value> {
        assert!(
            id.0 < self.entries.len(),
            "Using {id} but only have {} entries",
            self.entries.len()
        );
        &mut self.entries[id.0]
    }

    pub fn id(&mut self, key: &Key) -> Option<KeyId> {
        self.keys
            .binary_search_by_key(&key, |(_id, k)| k)
            .ok()
            .map(|idx| self.keys[idx].0)
    }
}

impl<Key, Value> LookupMap<Key, Value>
where
    Key: PartialEq + Ord + Debug + Clone,
    Value: Default,
{
    // Note: When cfg!(debug_assertions), this does an O(n) check for duplicate entries
    // This is O(1) otherwise.
    pub fn insert(&mut self, key: impl Into<Key>) -> KeyId {
        let key = key.into();
        let id = KeyId(self.keys.len());

        match self.keys.binary_search_by_key(&&key, |(_id, k)| k) {
            Err(idx) => self.keys.insert(idx, (id, key.clone())),
            Ok(idx) => unreachable!("Inserting {key:?} but is already in the map at {id:?}"),
        }

        self.entries.push(Entry {
            id,
            key,
            value: Value::default(),
        });

        id
    }

    pub fn new_entry(&mut self, key: impl Into<Key>) -> &mut Entry<Key, Value> {
        let key = key.into();
        let id = self.insert(key);
        self.entry(id)
    }

    pub fn insert_or_entry(&mut self, key: impl Into<Key>) -> &mut Entry<Key, Value> {
        let key = key.into();
        let id = self.id(&key).unwrap_or_else(|| self.insert(key));

        &mut self.entries[id.0]
    }
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn check_empty() {
        let mut lum: LookupMap<u8, u32> = LookupMap::new();

        assert_eq!(lum.len(), 0);
        assert!(lum.is_empty());

        // Check all the keys we COULD have and make sure we don't.
        for x in 0..u8::MAX {
            assert_eq!(lum.id(&x), None);
        }

        let mut iter = lum.entries();
        assert_eq!(
            iter.next(),
            None,
            "First entry of lum.entries() wasn't empty but should have been"
        );
    }

    #[test]
    fn check_basic_insert() {
        let mut lum: LookupMap<u8, u32> = LookupMap::new();

        let _id: KeyId = lum.insert(0);

        assert_eq!(lum.len(), 1);
        assert!(!lum.is_empty());

        assert_eq!(lum.id(&0), Some(KeyId(0)));
        // Check all the keys we COULD have too
        for x in 1..u8::MAX {
            assert_eq!(lum.id(&x), None);
        }

        let mut iter = lum.entries();
        assert_eq!(
            iter.next(),
            Some(&Entry {
                id: KeyId(0),
                key: 0,
                value: 0_u32
            }),
            "First entry of lum.entries() wasn't empty but should have been"
        );
        drop(iter);
    }

    #[test]
    fn check_basic_entrh() {
        let mut lum: LookupMap<u8, u32> = LookupMap::new();

        // insert 0 (at KeyId(0))
        let _id: KeyId = lum.insert(0);

        // insert 1 (at KeyId(1))
        let id: KeyId = lum.insert(1);
        assert_eq!(lum.len(), 2);
        assert!(!lum.is_empty());

        let entry = lum.entry(id);
        assert_eq!(
            entry,
            &Entry {
                id: KeyId(1),
                key: 1,
                value: 0_u32
            }
        );
    }
}
