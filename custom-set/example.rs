use std::collections::{Collection, Set, MutableSet};
use std::slice;

#[derive(Debug)]
pub struct CustomSet<T>(Vec<T>);

/// Create an empty set.
impl <T> CustomSet<T> {
    pub fn new() -> CustomSet<T> {
        CustomSet(vec!())
    }

    pub fn iter(&self) -> Items<T> {
        let &CustomSet(ref v) = self;
        Items(v.iter())
    }
}

pub struct Items<'a, T: 'a>(slice::Items<'a, T>);

impl<'a, T> Iterator<&'a T> for Items<'a, T> {
    fn next(&mut self) -> Option<&'a T> {
        let &Items(ref mut iter) = self;
        iter.next()
    }
}

impl<T: Eq> CustomSet<T> {
    fn find_index(&self, value: &T) -> Option<usize> {
        let &CustomSet(ref v) = self;
        for (idx, x) in v.iter().enumerate() {
            if x == value {
                return Some(idx)
            }
        }
        return None
    }
}

impl<T: Ord> CustomSet<T> {
    /// Visit the values representing the difference, in ascending order.
    ///
    /// The difference between set A and set B is all the items in set A that are not in set B.
    pub fn difference<'a>(&'a self, other: &'a CustomSet<T>) -> DifferenceItems<T> {
        DifferenceItems(box self
                        .diff_iter(other)
                        .filter(|&(_, diff)| diff == OnlyA)
                        .map(|(v, _)| v))
    }

    /// Visit the values representing the intersection, in ascending order.
    /// 
    /// The intersection of set A and set B is all the items in set A that are also in set B.
    pub fn intersection<'a>(&'a self, other: &'a CustomSet<T>) -> IntersectionItems<T> {
        IntersectionItems(box self
                          .diff_iter(other)
                          .filter(|&(_, diff)| diff == Both)
                          .map(|(v, _)| v))
    }
    
    /// Visit the values representing the union, in ascending order.
    /// 
    /// The union of set A and set B is all the items in both sets (with doubles removed).
    pub fn union<'a>(&'a self, other: &'a CustomSet<T>) -> UnionItems<T> {
        UnionItems(box self.diff_iter(other).map(|(v, _)| v))
    }
    
    fn diff_iter<'a>(&'a self, other: &'a CustomSet<T>) -> DiffIter<T> {
        let &CustomSet(ref self_v) = self;
        let &CustomSet(ref other_v) = other;
        DiffIter { a: CachingWrapper::wrap(self_v.iter()),
                   b: CachingWrapper::wrap(other_v.iter()) }
    }
}

// Those boxes are used just to avoid having to specify a complex Map<..., Filter<..., DiffIter>>
// nested iterator type.
pub struct DifferenceItems<'a, T>(Box<Iterator<&'a T> + 'a>);
pub struct IntersectionItems<'a, T>(Box<Iterator<&'a T> + 'a>);
pub struct UnionItems<'a, T>(Box<Iterator<&'a T> + 'a>);

impl<'a, T> Iterator<&'a T> for DifferenceItems<'a, T> {
    fn next(&mut self) -> Option<&'a T> {
        let &DifferenceItems(ref mut iter) = self;
        iter.next()
    }
}

impl<'a, T> Iterator<&'a T> for IntersectionItems<'a, T> {
    fn next(&mut self) -> Option<&'a T> {
        let &IntersectionItems(ref mut iter) = self;
        iter.next()
    }
}

impl<'a, T> Iterator<&'a T> for UnionItems<'a, T> {
    fn next(&mut self) -> Option<&'a T> {
        let &UnionItems(ref mut iter) = self;
        iter.next()
    }
}

impl<T> Collection for CustomSet<T> {
    fn len(&self) -> usize {
        let &CustomSet(ref v) = self;
        v.len()
    }

    fn is_empty(&self) -> bool {
        let &CustomSet(ref v) = self;
        v.is_empty()
    }
}

impl<T: Ord> Set<T> for CustomSet<T> {
    fn contains(&self, value: &T) -> bool {
        self.find_index(value).is_some()        
    }

    fn is_disjoint(&self, other: &CustomSet<T>) -> bool {
        // Traverse the ordered vectors, the big benefit of this over unordered vectors (which
        // would make the add method O(1) is that this method is O(n+m) where it would otherwise be
        // O(n*m). Not that for the chosen representation performance is actually anywhere near
        // decent, the standard lib sets are much much better.
        let mut diff_iter = self.diff_iter(other);
        for (_, diff) in diff_iter {
            match diff {
                Both => return false,
                _ => {}
            }
        }
        true
    }

    fn is_subset(&self, other: &CustomSet<T>) -> bool {
        let mut diff_iter = self.diff_iter(other);
        for (_, diff) in diff_iter {
            match diff {
                OnlyA => return false,
                _ => {}
            }
        }
        true
    }
}

impl<T> Mutable for CustomSet<T> {
    fn clear(&mut self) {
        let &CustomSet(ref mut v) = self;
        v.clear();
    }
}

enum InsertPos {
    Begin,
    Before(usize),
    End,
}

fn find_insert_pos<T: Ord>(v: &Vec<T>, value: &T) -> Option<InsertPos> {
    for (idx, x) in v.iter().enumerate() {
        match value.cmp(x) {
            Less if idx == 0 => return Some(Begin),
            Less => return Some(Before(idx)),
            Equal => return None,
            Greater => ()
        }
    }
    Some(End)
}

impl<T: Ord> MutableSet<T> for CustomSet<T> {
    fn insert(&mut self, value: T) -> bool {
        let &CustomSet(ref mut v) = self;
        match find_insert_pos(v, &value) {
            None => false,
            Some(Begin) => { v.insert(0, value); true },
            Some(Before(idx)) => { v.insert(idx-1, value); true },
            Some(End) => { v.push(value); true },
        }
    }

    fn remove(&mut self, value: &T) -> bool {
        let idx = match self.find_index(value) {
            None => return false,
            Some(idx) => idx 
        };
        let &CustomSet(ref mut v) = self;
        v.remove(idx);
        true
    }
}

impl<A: Ord> FromIterator<A> for CustomSet<A> {
    fn from_iter<T: Iterator<A>>(mut iterator: T) -> CustomSet<A> {
        let mut set: CustomSet<A> = CustomSet::new();
        for v in iterator {
            set.insert(v);
        }
        set
    }
}

// A wrapper around an iterator so that a single element can be viewed multiple times.
struct CachingWrapper<'a, T: 'a> {
    iter: slice::Items<'a, T>,
    cache: Option<&'a T>,
}

impl<'a, T> CachingWrapper<'a, T> {
    fn wrap(iter: slice::Items<'a, T>) -> CachingWrapper<'a, T> {
        let mut wrapper = CachingWrapper { iter: iter, cache: None };
        wrapper.advance();
        wrapper
    }
    fn current(&self) -> Option<&'a T> {
        self.cache
    }
    fn advance(&mut self) {
        self.cache = self.iter.next();
    }
}

#[derive(PartialEq, Eq)]
enum Diff {
    OnlyA,
    OnlyB,
    Both
}

// Iterator for the differences between two ordered vectors.
struct DiffIter<'a, T: 'a> {
    a: CachingWrapper<'a, T>,
    b: CachingWrapper<'a, T>,
}

impl<'a, T: Ord> Iterator<(&'a T, Diff)> for DiffIter<'a, T> {
    fn next(&mut self) -> Option<(&'a T, Diff)> {
        match (self.a.current(), self.b.current()) {
            (None, None) => None,
            (Some(a_val), None) => {
                self.a.advance();
                Some((a_val, OnlyA))
            },
            (None, Some(b_val)) => {
                self.b.advance();
                Some((b_val, OnlyB))
            },
            (Some(a_val), Some(b_val)) => {
                match (*a_val).cmp(b_val) {
                    Less => {
                        self.a.advance();
                        Some((a_val, OnlyA))
                    }
                    Equal => {
                        self.a.advance();
                        self.b.advance();
                        Some((a_val, Both))
                    }
                    Greater => {
                        self.b.advance();
                        Some((b_val, OnlyB))
                    }
                }
            }
        }
    }
}
