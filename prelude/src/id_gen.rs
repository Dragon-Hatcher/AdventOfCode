use rustc_hash::FxHashMap;
use std::{borrow::Borrow, hash::Hash};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id(i64);

#[derive(Debug)]
pub struct IdGen<T> {
    id_num: i64,
    map: FxHashMap<T, Id>,
}

impl<T> IdGen<T>
where
    T: Hash + Eq + Clone,
{
    pub fn new() -> Self {
        IdGen {
            id_num: 0,
            map: Default::default(),
        }
    }

    fn next_id(&mut self) -> Id {
        let id = Id(self.id_num);
        self.id_num += 1;
        id
    }

    pub fn get_id<Q>(&mut self, t: &Q) -> Id
    where
        T: Borrow<Q>,
        Q: Hash + Eq + ToOwned<Owned = T>,
    {
        if !self.map.contains_key(t) {
            let next_id = self.next_id();
            self.map.insert(t.to_owned(), next_id);
        }

        self.map[t]
    }

    pub fn all_ids(&self) -> impl Iterator<Item = Id> + '_ {
        self.map.values().copied()
    }
}

impl<T> Default for IdGen<T>
where
    T: Hash + Eq + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}
