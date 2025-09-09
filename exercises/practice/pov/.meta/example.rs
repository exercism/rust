use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tree<T: Debug + Ord> {
    label: T,
    children: Vec<Box<Tree<T>>>,
}

impl<T: Debug + Ord> Tree<T> {
    pub fn new(label: T) -> Self {
        Self {
            label,
            children: Default::default(),
        }
    }

    pub fn with_child(mut self, child: Self) -> Self {
        self.children.insert(
            self.children
                .binary_search_by(|c| c.label.cmp(&child.label))
                .unwrap_err(),
            Box::new(child),
        );
        self
    }

    pub fn pov_from(&mut self, from: &T) -> bool {
        self.pov_from_rec(from).is_some()
    }

    fn pov_from_rec(&mut self, from: &T) -> Option<Vec<usize>> {
        if &self.label == from {
            return Some(Vec::new());
        }

        // Run `pov_from_rec` over all children, until finding the one where it
        // worked. That also returns the list of indexes to traverse to find the
        // insertion point for the old POV.
        let (pos, mut index_list) = self
            .children
            .iter_mut()
            .enumerate()
            .find_map(|(i, child)| child.pov_from_rec(from).map(|index_list| (i, index_list)))?;

        // swap old and new POV
        let mut old_pov = self.children.remove(pos);
        std::mem::swap(self, &mut old_pov);

        // find parent of old POV
        let mut parent_of_old_pov = self;
        for i in index_list.iter().rev() {
            parent_of_old_pov = &mut parent_of_old_pov.children[*i];
        }

        // put old POV into its new place
        let new_idx = parent_of_old_pov
            .children
            .binary_search_by(|c| c.label.cmp(&old_pov.label))
            .unwrap_err();
        parent_of_old_pov.children.insert(new_idx, old_pov);

        // Record index of old POV such that other recursive calls can insert
        // their old POV as the child of ours.
        index_list.push(new_idx);

        Some(index_list)
    }

    pub fn path_to<'a>(&'a mut self, from: &'a T, to: &'a T) -> Option<Vec<&'a T>> {
        if !self.pov_from(to) {
            return None;
        }
        self.path_from(from)
    }

    fn path_from<'a>(&'a self, from: &'a T) -> Option<Vec<&'a T>> {
        if &self.label == from {
            return Some(vec![from]);
        }
        let mut path = self.children.iter().find_map(|c| c.path_from(from))?;
        path.push(&self.label);
        Some(path)
    }
}
