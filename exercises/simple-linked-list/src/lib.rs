pub struct SimpleLinkedList<T> {
    // Delete this field
    // _dummy is needed to avoid unused parameter error during compilation
    _dummy: T,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn len(&self) -> usize {
        unimplemented!()
    }

    pub fn push(&mut self, element: T) {
        unimplemented!()
    }

    pub fn pop(&mut self) -> Option<T> {
        unimplemented!()
    }

    pub fn peek(&self) -> Option<&T> {
        unimplemented!()
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        unimplemented!()
    }
}


impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(item: &[T]) -> Self {
        unimplemented!()
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        unimplemented!()
    }
}
