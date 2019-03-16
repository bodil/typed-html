use std::collections::BTreeMap;

#[derive(Clone)]
pub struct StringyMap<K, V>(BTreeMap<String, (K, V)>);

impl<K, V> StringyMap<K, V>
where
    K: ToString,
{
    pub fn new() -> Self {
        StringyMap(BTreeMap::new())
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        let s = k.to_string();
        self.0.insert(s, (k, v)).map(|(_, v)| v)
    }

    pub fn remove(&mut self, k: &K) -> Option<V> {
        let s = k.to_string();
        self.0.remove(&s).map(|(_, v)| v)
    }

    pub fn iter(&self) -> impl Iterator<Item = &(K, V)> {
        self.0.values()
    }

    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.0.values().map(|(k, _)| k)
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<K, V, OK, OV> From<Vec<(OK, OV)>> for StringyMap<K, V>
where
    OK: Into<K>,
    OV: Into<V>,
    K: ToString,
{
    fn from(vec: Vec<(OK, OV)>) -> Self {
        let mut out = Self::new();
        for (key, value) in vec {
            out.insert(key.into(), value.into());
        }
        out
    }
}
