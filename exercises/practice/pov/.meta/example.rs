use std::ops::{Deref, DerefMut};

#[derive(Clone, PartialEq)]
pub struct Tree<T: Clone + PartialEq> {
    label: T,
    children: Vec<Box<Tree<T>>>,
}

impl<T: Clone + PartialEq> Tree<T> {
    #[must_use]
    pub fn new(label: T) -> Self {
        Self {
            label,
            children: Default::default(),
        }
    }

    #[must_use]
    pub fn with_children(label: T, children: Vec<Self>) -> Self {
        Self {
            label,
            children: children.into_iter().map(Box::new).collect(),
        }
    }

    #[must_use]
    pub fn get_label(&self) -> T {
        self.label.clone()
    }

    #[must_use]
    pub fn get_children(&self) -> Vec<&Self> {
        self.children.iter().map(Box::deref).collect()
    }

    #[must_use]
    pub fn pov_from(&self, from: &T) -> Option<Self> {
        // list of (child, parent, child's index in parent.children)
        let mut lookup = vec![(self, None, None)];
        let mut stack = vec![self];
        while let Some(parent) = stack.pop() {
            if &parent.label == from {
                return self.reparent(parent, lookup.as_slice()).into();
            }
            lookup.extend(
                parent
                    .children
                    .iter()
                    .map(Box::deref)
                    .enumerate()
                    .map(|(i, child)| (child, Some(parent), Some(i))),
            );
            stack.extend(parent.children.iter().map(Box::deref));
        }
        None
    }

    // lookup is list of (child, parent, child's index in parent.children)
    #[must_use]
    fn reparent(&self, parent: &Self, lookup: &[(&Self, Option<&Self>, Option<usize>)]) -> Self {
        let mut new_root = parent.clone();
        let mut current = parent;
        let mut clone_mut = &mut new_root;
        let find_parent = |child| lookup.iter().find(|(c, _p, _i)| *c == child);
        while let Some(&(_, Some(parent), Some(index))) = find_parent(current) {
            let mut parent_clone = parent.clone();
            parent_clone.children.swap_remove(index);
            clone_mut.children.push(Box::new(parent_clone));
            current = parent;
            let new_box = clone_mut
                .children
                .last_mut()
                .expect("We just inserted node, this is not empty");
            clone_mut = new_box.deref_mut();
        }
        new_root
    }

    #[must_use]
    pub fn path_to(&self, from: &T, to: &T) -> Option<Vec<T>> {
        if from != &self.label {
            return self.pov_from(from).and_then(|pov| pov.path_to(from, to));
        }
        if to == &self.label {
            return Some(vec![self.label.clone()]);
        }
        for child in self.children.iter() {
            if let Some(mut path) = child.path_to(&child.label, to) {
                path.insert(0, self.label.clone());
                return Some(path);
            }
        }
        None
    }
}
