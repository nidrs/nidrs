#![doc = include_str!("../readme.md")]
use std::{
    any::{Any, TypeId},
    collections::{HashMap, HashSet},
    fmt::Debug,
    ops::{Deref, DerefMut},
    sync::Arc,
};

#[derive(Debug, Clone)]
pub struct RefMeta(pub Arc<Metamap>);

impl RefMeta {
    pub fn new(meta: Metamap) -> Self {
        RefMeta(Arc::new(meta))
    }
}

impl Deref for RefMeta {
    type Target = Metamap;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RefMeta {
    fn deref_mut(&mut self) -> &mut Self::Target {
        Arc::get_mut(&mut self.0).expect("Cannot mutate a shared reference")
    }
}

/// Represents a Metamap object.
///
/// This struct contains a map that associates keys of type `String` with values of type `Box<dyn Any + Send + Sync>`.
/// It also has an optional field `extend` that holds a boxed `Metamap` object.
#[derive(Default)]
pub struct Metamap {
    map: HashMap<String, Box<dyn Any + Send + Sync>>,
    extend: Option<RefMeta>,
}

impl Debug for Metamap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut keys = self.map.keys().collect::<HashSet<&String>>();
        if let Some(p) = &self.extend {
            let keys2 = p.0.map.keys().collect::<HashSet<&String>>();
            keys.extend(keys2);
        }
        f.debug_struct("Metamap").field("keys", &keys.iter()).finish()
    }
}

impl Metamap {
    pub fn new() -> Self {
        Metamap { map: HashMap::new(), extend: None }
    }

    pub fn inner(&self) -> &HashMap<String, Box<dyn Any + Send + Sync>> {
        &self.map
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Metamap { map: HashMap::with_capacity(capacity), extend: None }
    }

    pub fn capacity(&self) -> usize {
        self.map.capacity()
    }

    pub fn set<V: Any + Send + Sync>(&mut self, key: impl Into<String>, value: V) -> &mut Self {
        self.map.insert(key.into(), Box::new(value));
        self
    }

    pub fn get<V: Any + Send + Sync>(&self, key: impl Into<String>) -> Option<&V> {
        let key: String = key.into();
        let t = self.map.get(&key).and_then(|v| v.downcast_ref::<V>());
        let r = match t {
            Some(v) => Some(v),
            None => match &self.extend {
                Some(p) => p.get::<V>(key),
                None => None,
            },
        };
        r
    }

    pub fn get_mut<V: Any + Send + Sync>(&mut self, key: impl Into<String>) -> Option<&mut V> {
        let key = key.into();
        let t = self.map.get_mut(&key).and_then(|v| v.downcast_mut::<V>());
        match t {
            Some(v) => Some(v),
            None => None,
        }
    }

    pub fn set_data<V: Any + Send + Sync>(&mut self, value: V) -> &mut Self {
        self.set(type_key::<V>(), value)
    }

    pub fn get_data<V: Any + Send + Sync>(&self) -> Option<&V> {
        self.get(type_key::<V>())
    }

    pub fn get_mut_data<V: Any + Send + Sync>(&mut self) -> Option<&mut V> {
        self.get_mut(type_key::<V>())
    }

    pub fn take<V: Any + Send + Sync>(&mut self, key: impl Into<String>) -> Option<V> {
        let key: String = key.into();
        let t = self.map.remove(&key).and_then(|v| v.downcast::<V>().ok());
        t.map(|v| *v)
    }

    pub fn take_data<V: Any + Send + Sync>(&mut self) -> Option<V> {
        self.take(type_key::<V>())
    }

    pub fn contains(&self, key: impl Into<String>) -> bool {
        self.map.contains_key(&key.into())
    }

    pub fn contains_value<V: Any + Send + Sync>(&self) -> bool {
        self.contains(type_key::<V>())
    }

    pub fn remove(&mut self, key: impl Into<String>) -> Option<Box<dyn Any + Send + Sync>> {
        let key: String = key.into();
        let t = self.map.remove(&key);
        match t {
            Some(v) => Some(v),
            None => None,
        }
    }

    pub fn remove_value<V: Any + Send + Sync>(&mut self) -> Option<Box<dyn Any + Send + Sync>> {
        self.remove(type_key::<V>())
    }

    pub fn merge(&mut self, meta: Metamap) -> &mut Self {
        for (k, v) in meta.map {
            self.map.insert(k, v);
        }
        self
    }

    pub fn extend(&mut self, ref_meta: RefMeta) -> &mut Self {
        self.extend = Some(ref_meta);
        self
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<String, Box<dyn Any + Send + Sync>> {
        self.map.iter()
    }

    pub fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<String, Box<dyn Any + Send + Sync>> {
        self.map.iter_mut()
    }

    pub fn keys(&self) -> Vec<String> {
        let mut keys = self.map.keys().collect::<HashSet<&String>>();
        if let Some(p) = &self.extend {
            let keys2 = p.map.keys().collect::<HashSet<&String>>();
            keys.extend(keys2);
        }
        keys.iter().map(|k| k.to_string()).collect()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }
}

pub fn type_key<T: 'static>() -> String {
    format!("{:?}", TypeId::of::<T>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata() {
        #[derive(Debug, PartialEq, Eq)]
        enum TestEnum {
            A,
            B,
        }

        #[derive(Debug, PartialEq, Eq)]
        struct TestData {
            pub name: String,
        }

        #[derive(Debug, PartialEq, Eq)]
        struct TupleData(i32, String);

        let mut meta = Metamap::new();
        meta.set_data(TestEnum::A);
        meta.set_data(TestData { name: "test".to_string() });
        meta.set_data(TupleData(1, "tuple".to_string()));

        assert_eq!(*meta.get_data::<TestEnum>().unwrap(), TestEnum::A);
        assert_ne!(*meta.get_data::<TestEnum>().unwrap(), TestEnum::B);
        assert_eq!(*meta.get_data::<TestData>().unwrap(), TestData { name: "test".to_string() });
        assert_eq!(*meta.get_data::<TupleData>().unwrap(), TupleData(1, "tuple".to_string()));

        assert_eq!(meta.take_data::<TestData>().unwrap(), TestData { name: "test".to_string() });
        assert_eq!(meta.take_data::<TestEnum>().unwrap(), TestEnum::A);
        assert_eq!(meta.take_data::<TupleData>().unwrap(), TupleData(1, "tuple".to_string()));

        assert!(meta.get_data::<TestData>().is_none());
        assert!(meta.get_data::<TestEnum>().is_none());
        assert!(meta.get_data::<TupleData>().is_none());
    }

    #[test]
    fn test_meta() {
        let mut meta = Metamap::new();
        meta.set("a", 1);
        meta.set("b", "2");
        meta.set("c", 3.0);
        meta.set("d", "4".to_string());
        meta.set("e", vec![1, 2, 3]);
        meta.set("f", vec!["1", "2", "3"]);
        meta.set("g", vec![1.0, 2.0, 3.0]);
        meta.set("h", vec!["1".to_string(), "2".to_string(), "3".to_string()]);
        meta.set("i", vec![vec![1, 2], vec![3, 4]]);
        meta.set("j", vec![vec!["1", "2"], vec!["3", "4"]]);
        meta.set("k", vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        meta.set("l", vec![vec!["1".to_string(), "2".to_string()], vec!["3".to_string(), "4".to_string()]]);

        assert_eq!(*meta.get::<i32>("a").unwrap(), 1);
        assert_eq!(*meta.get::<&str>("b").unwrap(), "2");
        assert_eq!(*meta.get::<f64>("c").unwrap(), 3.0);
        assert_eq!(*meta.get::<String>("d").unwrap(), "4".to_string());
        assert_eq!(*meta.get::<Vec<i32>>("e").unwrap(), vec![1, 2, 3]);
        assert_eq!(*meta.get::<Vec<&str>>("f").unwrap(), vec!["1", "2", "3"]);
        assert_eq!(*meta.get::<Vec<f64>>("g").unwrap(), vec![1.0, 2.0, 3.0]);
        assert_eq!(*meta.get::<Vec<String>>("h").unwrap(), vec!["1".to_string(), "2".to_string(), "3".to_string()]);
        assert_eq!(*meta.get::<Vec<Vec<i32>>>("i").unwrap(), vec![vec![1, 2], vec![3, 4]]);
        assert_eq!(*meta.get::<Vec<Vec<&str>>>("j").unwrap(), vec![vec!["1", "2"], vec!["3", "4"]]);
    }

    #[test]
    fn test_mut_meta() {
        let mut meta = Metamap::new();
        meta.set("a", 1);
        meta.set("b", "2");
        meta.set("c", 3.0);
        meta.set("d", "4".to_string());
        meta.set("e", vec![1, 2, 3]);
        meta.set("f", vec!["1", "2", "3"]);
        meta.set("g", vec![1.0, 2.0, 3.0]);
        meta.set("h", vec!["1".to_string(), "2".to_string(), "3".to_string()]);
        meta.set("i", vec![vec![1, 2], vec![3, 4]]);
        meta.set("j", vec![vec!["1", "2"], vec!["3", "4"]]);
        meta.set("k", vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        meta.set("l", vec![vec!["1".to_string(), "2".to_string()], vec!["3".to_string(), "4".to_string()]]);

        *meta.get_mut::<i32>("a").unwrap() = 2;
        *meta.get_mut::<&str>("b").unwrap() = "3";
        *meta.get_mut::<f64>("c").unwrap() = 4.0;
        *meta.get_mut::<String>("d").unwrap() = "5".to_string();
        *meta.get_mut::<Vec<i32>>("e").unwrap() = vec![2, 3, 4];
        *meta.get_mut::<Vec<&str>>("f").unwrap() = vec!["2", "3", "4"];
        *meta.get_mut::<Vec<f64>>("g").unwrap() = vec![2.0, 3.0, 4.0];
        *meta.get_mut::<Vec<String>>("h").unwrap() = vec!["2".to_string(), "3".to_string(), "4".to_string()];
        *meta.get_mut::<Vec<Vec<i32>>>("i").unwrap() = vec![vec![2, 3], vec![4, 5]];
        *meta.get_mut::<Vec<Vec<&str>>>("j").unwrap() = vec![vec!["2", "3"], vec!["4", "5"]];

        assert_eq!(*meta.get::<i32>("a").unwrap(), 2);
        assert_eq!(*meta.get::<&str>("b").unwrap(), "3");
        assert_eq!(*meta.get::<f64>("c").unwrap(), 4.0);
        assert_eq!(*meta.get::<String>("d").unwrap(), "5".to_string());
        assert_eq!(*meta.get::<Vec<i32>>("e").unwrap(), vec![2, 3, 4]);
        assert_eq!(*meta.get::<Vec<&str>>("f").unwrap(), vec!["2", "3", "4"]);
        assert_eq!(*meta.get::<Vec<f64>>("g").unwrap(), vec![2.0, 3.0, 4.0]);
        assert_eq!(*meta.get::<Vec<String>>("h").unwrap(), vec!["2".to_string(), "3".to_string(), "4".to_string()]);
        assert_eq!(*meta.get::<Vec<Vec<i32>>>("i").unwrap(), vec![vec![2, 3], vec![4, 5]]);
        assert_eq!(*meta.get::<Vec<Vec<&str>>>("j").unwrap(), vec![vec!["2", "3"], vec!["4", "5"]]);
    }

    #[test]
    fn test_meta_merge() {
        let mut meta1 = Metamap::new();
        meta1.set("a", 1);
        meta1.set("b", "2");
        meta1.set("c", 3.0);
        meta1.set("d", "4".to_string());
        meta1.set("e", vec![1, 2, 3]);
        meta1.set("f", vec!["1", "2", "3"]);
        meta1.set("g", vec![1.0, 2.0, 3.0]);
        meta1.set("h", vec!["1".to_string(), "2".to_string(), "3".to_string()]);
        meta1.set("i", vec![vec![1, 2], vec![3, 4]]);
        meta1.set("j", vec![vec!["1", "2"], vec!["3", "4"]]);
        meta1.set("k", vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        meta1.set("l", vec![vec!["1".to_string(), "2".to_string()], vec!["3".to_string(), "4".to_string()]]);

        let mut meta2 = Metamap::new();
        meta2.set("e", vec![2, 3, 4]);
        meta2.set("f", vec!["2", "3", "4"]);
        meta2.set("g", vec![2.0, 3.0, 4.0]);
        meta2.set("h", vec!["2".to_string(), "3".to_string(), "4".to_string()]);
        meta2.set("i", vec![vec![2, 3], vec![4, 5]]);
        meta2.set("j", vec![vec!["2", "3"], vec!["4", "5"]]);

        meta1.merge(meta2);

        assert_eq!(*meta1.get::<i32>("a").unwrap(), 1);
        assert_eq!(*meta1.get::<&str>("b").unwrap(), "2");
        assert_eq!(*meta1.get::<f64>("c").unwrap(), 3.0);
        assert_eq!(*meta1.get::<String>("d").unwrap(), "4".to_string());
        assert_eq!(*meta1.get::<Vec<i32>>("e").unwrap(), vec![2, 3, 4]);
        assert_eq!(*meta1.get::<Vec<&str>>("f").unwrap(), vec!["2", "3", "4"]);
        assert_eq!(*meta1.get::<Vec<f64>>("g").unwrap(), vec![2.0, 3.0, 4.0]);
        assert_eq!(*meta1.get::<Vec<String>>("h").unwrap(), vec!["2".to_string(), "3".to_string(), "4".to_string()]);
        assert_eq!(*meta1.get::<Vec<Vec<i32>>>("i").unwrap(), vec![vec![2, 3], vec![4, 5]]);
        assert_eq!(*meta1.get::<Vec<Vec<&str>>>("j").unwrap(), vec![vec!["2", "3"], vec!["4", "5"]]);
    }

    #[test]
    fn test_meta_extend() {
        let mut meta1 = Metamap::new();
        meta1.set("a", 1);
        meta1.set("b", "2");
        meta1.set("c", 3.0);
        meta1.set("d", "4".to_string());
        meta1.set("e", vec![1, 2, 3]);
        meta1.set("f", vec!["1", "2", "3"]);
        meta1.set("g", vec![1.0, 2.0, 3.0]);

        let mut meta2 = Metamap::new();
        meta2.set("e", vec![2, 3, 4]);
        meta2.set("i", vec![vec![2, 3], vec![4, 5]]);
        meta2.set("j", vec![vec!["2", "3"], vec!["4", "5"]]);

        meta1.extend(RefMeta::new(meta2));

        assert_eq!(*meta1.get::<i32>("a").unwrap(), 1);
        assert_eq!(*meta1.get::<&str>("b").unwrap(), "2");
        assert_eq!(*meta1.get::<f64>("c").unwrap(), 3.0);
        assert_eq!(*meta1.get::<String>("d").unwrap(), "4".to_string());
        assert_eq!(*meta1.get::<Vec<i32>>("e").unwrap(), vec![1, 2, 3]);
        assert_eq!(*meta1.get::<Vec<&str>>("f").unwrap(), vec!["1", "2", "3"]);
        assert_eq!(*meta1.get::<Vec<f64>>("g").unwrap(), vec![1.0, 2.0, 3.0]);
        assert_eq!(*meta1.get::<Vec<Vec<i32>>>("i").unwrap(), vec![vec![2, 3], vec![4, 5]]);
        assert_eq!(*meta1.get::<Vec<Vec<&str>>>("j").unwrap(), vec![vec!["2", "3"], vec!["4", "5"]]);
    }
}
