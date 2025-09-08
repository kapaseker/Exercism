use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T>(HashSet<T>)
where
    T: Eq + PartialEq + Hash;

impl<T> CustomSet<T>
where
    T: Copy + Eq + PartialEq + Hash,
{
    pub fn new(_input: &[T]) -> Self {
        let mut vec = HashSet::new();
        for x in _input {
            vec.insert(*x);
        }
        Self(vec)
    }

    pub fn contains(&self, _element: &T) -> bool {
        self.0.contains(_element)
    }

    pub fn add(&mut self, _element: T) {
        self.0.insert(_element);
    }

    pub fn is_subset(&self, _other: &Self) -> bool {
        self.0.iter().all(|s| _other.0.contains(s))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        self.intersection_set(_other).is_empty()
    }

    pub fn intersection(&self, _other: &Self) -> Self {
        Self(self.intersection_set(_other))
    }

    pub fn difference(&self, _other: &Self) -> Self {
        Self(HashSet::from_iter(self.0.iter().filter(|&s| !_other.0.contains(s)).copied()))
    }

    pub fn union(&self, _other: &Self) -> Self {
        let mut set = HashSet::new();

        for x in _other.0.iter() {
            set.insert(*x);
        }

        for x in self.0.iter() {
            set.insert(*x);
        }

        Self(set)
    }

    fn intersection_set(&self, _other: &Self) -> HashSet<T> {
        HashSet::from_iter(_other.0.iter().filter(|&s| self.0.contains(s)).copied())
    }
}
