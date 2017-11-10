use std::cmp::Ordering;
use std::collections::{BTreeSet, HashSet};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem;
use std::cell::RefCell;

extern crate noisy_float;
use noisy_float::prelude::*;

type Book = usize;
type GroupedBasket = Vec<Group>;
type Price = f64;
const BOOK_PRICE: Price = 8.0;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Group(RefCell<BTreeSet<Book>>);

impl Group {
    fn new() -> Group {
        Group(RefCell::new(BTreeSet::new()))
    }

    fn new_containing(book: Book) -> Group {
        let g = Group::new();
        g.0.borrow_mut().insert(book);
        g
    }

    fn price(&self) -> Price {
        (self.0.borrow().len() as Price) * BOOK_PRICE *
            match self.0.borrow().len() {
                2 => 0.95,
                3 => 0.90,
                4 => 0.80,
                5 => 0.75,
                _ => 1.0,
            }
    }
}


impl Ord for Group {
    // we want to order groups first by qty contained DESC, then by lowest value ASC
    fn cmp(&self, other: &Group) -> Ordering {
        match other.0.borrow().len().cmp(&self.0.borrow().len()) {
            Ordering::Equal => {
                if self.0.borrow().len() == 0 {
                    Ordering::Equal
                } else {
                    self.0.borrow().iter().next().unwrap().cmp(
                        other
                            .0
                            .borrow()
                            .iter()
                            .next()
                            .unwrap(),
                    )
                }
            }
            otherwise => otherwise,
        }
    }
}

impl PartialOrd for Group {
    fn partial_cmp(&self, other: &Group) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for Group {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.0.borrow().hash(hasher);
    }
}

fn basket_price(basket: &GroupedBasket) -> Price {
    basket.iter().map(|g| g.price()).sum()
}

/// Compute the hash of a GroupedBasket
///
/// Note that we don't actually care at all about the _values_ within
/// the groups, only their lengths. Therefore, let's hash not the actual
/// GB but its lengths.
fn hash_of(basket: &GroupedBasket) -> u64 {
    let lengths = basket
        .iter()
        .map(|g| g.0.borrow().len())
        .collect::<Vec<_>>();
    let mut hasher = DefaultHasher::new();
    lengths.hash(&mut hasher);
    hasher.finish()
}

pub fn lowest_price(books: &[Book]) -> Price {
    DecomposeGroups::new(books)
        .map(|gb| r64(basket_price(&gb)))
        .min()
        .map(|r| r.raw())
        .unwrap_or(0.0)
}

struct DecomposeGroups {
    prev_states: HashSet<u64>,
    next: Option<GroupedBasket>,
}

impl Iterator for DecomposeGroups {
    type Item = GroupedBasket;
    fn next(&mut self) -> Option<Self::Item> {
        // our goal here: produce a stream of valid groups, differentiated by their
        // counts, from most compact to most dispersed.
        //
        // Algorithm:
        //  - Start with the most compact groups possible
        //  - If the number of groups == 0 or the max population of any group == 1, return None
        //  - For every item in the most populous group:
        //      - Try removing it and adding it to a smaller group.
        //          - Can any smaller group accept it? if yes, move it there and return
        //  - If it cannot be added to any smaller group, try the next item from this set
        //  - If no item from the most populous group can be added to any smaller group,
        //    then move the last item from the most populous group into a new group, alone,
        //    and return
        let return_value = self.next.clone();
        if let Some(groups) = mem::replace(&mut self.next, None) {
            if !(groups.is_empty() || groups.iter().all(|g| g.0.borrow().len() == 1)) {
                let mut hypothetical;
                for mpg_book in groups[0].0.borrow().iter() {
                    for (idx, other_group) in groups[1..].iter().enumerate() {
                        if !other_group.0.borrow().contains(mpg_book) {
                            hypothetical = groups.clone();
                            hypothetical[0].0.borrow_mut().remove(mpg_book);
                            hypothetical[1 + idx].0.borrow_mut().insert(*mpg_book);
                            hypothetical.sort();
                            let hypothetical_hash = hash_of(&hypothetical);
                            if !self.prev_states.contains(&hypothetical_hash) {
                                self.prev_states.insert(hypothetical_hash);
                                mem::replace(&mut self.next, Some(hypothetical));
                                return return_value;
                            }
                        }
                    }
                }
                // we've gone through all the items of the most populous group,
                // and none of them can be added to any other existing group.
                // We need to create a new group;
                let book = {
                    let backing_bt = groups[0].0.borrow();
                    let mut book_iter = backing_bt.iter();
                    book_iter.next().unwrap().clone()
                };
                hypothetical = groups.clone();
                hypothetical[0].0.borrow_mut().remove(&book);
                hypothetical.push(Group::new_containing(book));
                hypothetical.sort();
                self.prev_states.insert(hash_of(&hypothetical));
                mem::replace(&mut self.next, Some(hypothetical));
            }
        }
        return_value
    }
}

impl DecomposeGroups {
    fn new(books: &[Book]) -> DecomposeGroups {
        let mut book_groups = GroupedBasket::new();
        'nextbook: for book in books {
            for idx in 0..book_groups.len() {
                if !book_groups[idx].0.borrow().contains(&book) {
                    book_groups[idx].0.borrow_mut().insert(*book);
                    continue 'nextbook;
                }
            }
            // if we're here, we still haven't found a place for the book.
            // better add it to a new group
            book_groups.push(Group::new_containing(*book));
        }
        book_groups.sort();

        DecomposeGroups {
            next: Some(book_groups),
            prev_states: HashSet::new(),
        }
    }
}
