#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T> {
    // We fake using T here, so the compiler does not complain that
    // "parameter `T` is never used". Delete when no longer needed.
    phantom: std::marker::PhantomData<T>,
}

impl<T> CustomSet<T> {
    pub fn new(_input: &[T]) -> Self {
        todo!();
    }

    pub fn contains(&self, _element: &T) -> bool {
        todo!();
    }

    pub fn add(&mut self, _element: T) {
        todo!();
    }

    pub fn is_subset(&self, _other: &Self) -> bool {
        todo!();
    }

    pub fn is_empty(&self) -> bool {
        todo!();
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        todo!();
    }

    #[must_use]
    pub fn intersection(&self, _other: &Self) -> Self {
        todo!();
    }

    #[must_use]
    pub fn difference(&self, _other: &Self) -> Self {
        todo!();
    }

    #[must_use]
    pub fn union(&self, _other: &Self) -> Self {
        todo!();
    }
}
