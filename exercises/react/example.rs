// TODO: For now, lib is symlinked to example to ease local development.
// But the final plan is to provide a stub file once we know what the interface will be.

pub trait Cell<T> {
    fn value(&self) -> &T;
}

pub struct Reactor;

pub struct InputCell<T> {
    val: T,
}

pub struct Compute1Cell<'a, T: 'a, U, F: Fn(&T) -> U> {
    compute: F,
    cell: &'a Cell<T>,
    val: U,
}

impl Reactor {
    pub fn new() -> Reactor {
        Reactor{}
    }

    pub fn create_input<T>(&self, initial: T) -> InputCell<T> {
        InputCell {
            val: initial,
        }
    }

    pub fn create_compute1<'a, T, U, F>(&self, cell: &'a Cell<T>, compute: F) -> Compute1Cell<'a, T, U, F>
        where F: Fn(&T) -> U {
        Compute1Cell {
            val: compute(cell.value()),
            cell: cell,
            compute: compute,
        }
    }
}

impl <T> Cell<T> for InputCell<T> {
    fn value(&self) -> &T {
        &self.val
    }
}

impl <T> InputCell<T> {
    pub fn set_value(&mut self, new_val: T) {
        self.val = new_val;
    }
}

impl <'a, T, U, F: Fn(&T) -> U> Cell<U> for Compute1Cell<'a, T, U, F> {
    fn value(&self) -> &U {
        &self.val
    }
}
