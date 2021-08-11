use std::{
    hash::Hash,
    collections::{HashMap, HashSet},
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OneToManyMap<A: Eq + Hash + Clone, B: Eq + Hash + Clone> {
    to_bs: HashMap<A, HashSet<B>>,
    to_a: HashMap<B, A>,
    empty: HashSet<B>,
}
impl<A: Eq + Hash + Clone, B: Eq + Hash + Clone> Default for OneToManyMap<A, B> {
    fn default() -> Self {
        Self { to_bs: Default::default(), to_a: Default::default(), empty: Default::default() }
    }
}
impl<A: Eq + Hash + Clone, B: Eq + Hash + Clone> OneToManyMap<A, B> {
    pub fn insert(&mut self, a: A, b: B) -> Result<bool, (A, B)> {
        let bs = self.to_bs.entry(a.clone()).or_insert_with(HashSet::default);
        match self.to_a.get_mut(&b) {
            Some(a_element) if a_element == &a && bs.contains(&b) => {
                // providing a mapping we already had
                Ok(false)
            }
            None if !bs.contains(&b) => {
                // new a <-> b mapping that doesn't cause any conflicts
                bs.insert(b.clone());
                self.to_a.insert(b, a);
                Ok(true)
            }
            _ => Err((a, b)),
        }
    }
    pub fn contains(&self, a: &A, b: &B) -> bool {
        self.get_one(b) == Some(a)
    }
    pub fn get_one(&self, b: &B) -> Option<&A> {
        self.to_a.get(b)
    }
    pub fn get_many(&self, a: &A) -> &HashSet<B> {
        self.to_bs.get(a).unwrap_or(&self.empty)
    }
    pub fn remove_one(&mut self, b: &B) -> Option<A> {
        self.to_a.remove(b).map(|a| {
            assert!(self.to_bs.get_mut(&a).unwrap().remove(b));
            a
        })
    }
    pub fn remove_many(&mut self, a: &A) -> HashSet<B> {
        self.to_bs
            .remove(a)
            .map(|bs| {
                for b in bs.iter() {
                    self.to_a.remove(b).unwrap();
                }
                bs
            })
            .unwrap_or_else(Default::default)
    }
}
