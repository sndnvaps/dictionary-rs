use std::borrow::Borrow;
use std::clone::Clone;
use std::fmt;
use std::fmt::Write;
use std::hash::Hash;

use indexmap::IndexMap;

#[derive(PartialEq)]
pub struct Dictionary<K, V>
where
    K: Hash + Eq,
{
    table: IndexMap<K, V>,
}

#[allow(dead_code)]
impl<K, V> Dictionary<K, V>
where
    K: Clone + Hash + PartialEq + Copy + Eq,
    V: Clone + Copy,
{
    pub fn new() -> Dictionary<K, V> {
        Dictionary {
            table: IndexMap::new(),
        }
    }

    pub fn with_capacity(size: usize) -> Dictionary<K, V> {
        if size == 0 {
            panic!("Cannot create a zero-sized dict");
        }

        Dictionary {
            table: IndexMap::with_capacity(size),
        }
    }

    // Remove all key-value pairs in the map, while preserving its capacity.
    pub fn clear(&mut self) {
        self.table.clear()
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.table.contains_key(key)
    }

    //Returns a Result::Err if the vectors are different sizes
    pub fn from_vecs(mut key_vec: Vec<K>, mut value_vec: Vec<V>) -> Dictionary<K, V> {
        if key_vec.len() != value_vec.len() {
            panic!("Differently sized vecs");
        } else if key_vec.is_empty() {
            panic!("Cannot create a zero-sized dict");
        } else {
            let mut dict: Dictionary<K, V> = Dictionary::with_capacity(key_vec.len());
            for _ in 0..key_vec.len() {
                let key = key_vec.pop().unwrap();
                let value = value_vec.pop().unwrap();
                dict.insert(key, value);
            }

            dict
        }
    }

    pub fn from_tuples(tuples: Vec<(K, V)>) -> Dictionary<K, V> {
        if tuples.is_empty() {
            panic!("Cannot create a zero-sized vec");
        }
        let mut dict: Dictionary<K, V> = Dictionary::with_capacity(tuples.len());

        for (key, value) in tuples {
            dict.insert(key, value);
        }

        dict
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let val = self.table.get(key);
        return val;
    }

    pub fn get_full<Q>(&self, key: &Q) -> Option<(usize, &K, &V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let val = self.table.get_full(key);
        return val;
    }

    pub fn get_index_of<Q>(&self, key: &Q) -> Option<usize>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let val = self.table.get_index_of(key);
        return val;
    }

    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let val = self.table.get_key_value(key);
        return val;
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.table.insert(key, value)
    }

    pub fn iter(&self) -> indexmap::map::Iter<K, V> {
        self.table.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.table.is_empty()
    }

    pub fn len(&self) -> usize {
        self.table.len()
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let val = self.table.remove(key);
        return val;
    }

    pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.table.remove_entry(key)
    }
}

impl<K, V> fmt::Display for Dictionary<K, V>
where
    K: fmt::Display + Clone + Copy + Hash + Eq,
    V: fmt::Display + Clone + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output_str = String::new();
        output_str.push_str("{");

        for k in self.table.iter()
        // Iterate over all buckets containing an entry
        {
            write!(output_str, "{}: {}, ", k.0, k.1)?;
        }

        let len = output_str.len();
        if len > 1 {
            output_str = String::from(&output_str[..len - 2]);
        }
        output_str.push_str("}");

        write!(f, "{}", output_str)
    }
}

impl<K, V> fmt::Debug for Dictionary<K, V>
where
    K: fmt::Display + Clone + Copy + Hash + fmt::Debug + Eq,
    V: fmt::Display + Clone + Copy + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output_str = String::new();
        output_str.push_str("{");

        for k in self.table.iter()
        // Iterate over all buckets containing an entry
        {
            write!(output_str, "{}: {}, ", k.0, k.1)?;
        }

        let len = output_str.len();
        if len > 1 {
            output_str = String::from(&output_str[..len - 2]);
        }
        output_str.push_str("}");

        write!(f, "{}", output_str)
    }
}

impl<'a, K, V> FromIterator<&'a (K, V)> for Dictionary<K, V>
where
    K: Clone + Copy + Hash + std::cmp::PartialEq + Eq + 'a,
    V: Clone + Copy + std::cmp::PartialEq + 'a,
{
    fn from_iter<T: IntoIterator<Item = &'a (K, V)>>(iter: T) -> Self {
        let mut dict: Dictionary<K, V> = Dictionary::new();
        for (k, v) in iter {
            let _ = dict.insert(*k, *v);
        }
        dict
    }
}

impl<K, V> FromIterator<(K, V)> for Dictionary<K, V>
where
    K: Hash + Eq + Copy,
    V: Copy,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut dict: Dictionary<K, V> = Dictionary::new();
        for (k, v) in iter {
            let _ = dict.insert(k, v);
        }
        dict
    }
}

impl<K, V, X> indexmap::Equivalent<X> for Dictionary<K, V>
where
    Dictionary<K, V>: PartialEq<X>,
    K: Eq + Hash,
{
    fn equivalent(&self, other: &X) -> bool {
        *self == *other
    }
}
