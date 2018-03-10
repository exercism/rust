#[derive(Debug)]
pub struct CustomSet<T> {
    collection: Vec<T>,
}

impl<T: Ord + Clone> PartialEq for CustomSet<T> {
    fn eq(&self, other: &Self) -> bool {
        self.collection.iter().all(|x| other.contains(&x)) &&
        other.collection.iter().all(|x| self.contains(&x))
    }
}

impl<T: Ord + Clone> CustomSet<T> {
    pub fn new<'a, I>(inputs: I) -> CustomSet<T> where I: IntoIterator<Item = &'a T>, T: 'a {
        let mut s = CustomSet { collection: Vec::new() };
        for input in inputs {
            s.add(input.clone());
        }
        s
    }

    pub fn add(&mut self, element: T) {
        if !self.contains(&element) {
            self.collection.push(element)
        }
    }

    pub fn contains(&self, other: &T) -> bool {
        self.collection.contains(other)
    }

    pub fn is_empty(&self) -> bool {
        self.collection.is_empty()
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.collection.iter().all(|x| other.contains(x))
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        !self.collection.iter().any(|x| other.contains(x))
    }

    pub fn intersection(&self, other: &Self) -> CustomSet<T> {
        CustomSet::new(self.collection
                           .iter()
                           .filter(|c| other.contains(*c))
                           )
    }

    pub fn union(&self, other: &Self) -> CustomSet<T> {
        CustomSet::new(self.collection
                           .iter()
                           .chain(other.collection.iter())
                           )
    }

    pub fn difference(&self, other: &Self) -> CustomSet<T> {
        CustomSet::new(self.collection
                           .iter()
                           .filter(|c| !other.contains(*c))
                           )
    }
}
