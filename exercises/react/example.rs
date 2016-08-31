// TODO: For now, lib is symlinked to example to ease local development.
// But the final plan is to provide a stub file once we know what the interface will be.


use std::cell::Cell as MutCell;

pub trait Cell<T> {
    fn value(&self) -> &T;
    fn epoch(&self) -> usize;
}

pub struct Reactor;

pub struct InputCell<T> {
    val: T,
    epoch: usize,
}

pub struct Compute1Cell<'a, T: 'a, U: Copy, F: Fn(&T) -> U> {
    compute: F,
    cell: &'a Cell<T>,
    epoch: MutCell<usize>,
    val: MutCell<U>,
}

impl Reactor {
    pub fn new() -> Reactor {
        Reactor{}
    }

    pub fn create_input<T>(&self, initial: T) -> InputCell<T> {
        InputCell {
            val: initial,
            epoch: 0,
        }
    }

    pub fn create_compute1<'a, T, U: Copy, F: Fn(&T) -> U>(&self, cell: &'a Cell<T>, compute: F) -> Compute1Cell<'a, T, U, F> {
        Compute1Cell {
            val: MutCell::new(compute(cell.value())),
            cell: cell,
            epoch: MutCell::new(cell.epoch()),
            compute: compute,
        }
    }
}

impl <T> Cell<T> for InputCell<T> {
    fn value(&self) -> &T {
        &self.val
    }

    fn epoch(&self) -> usize {
        self.epoch
    }
}

impl <T> InputCell<T> {
    pub fn set_value(&mut self, new_val: T) {
        self.val = new_val;
    }
}

impl <'a, T, U: Copy, F: Fn(&T) -> U> Cell<U> for Compute1Cell<'a, T, U, F> {
    fn value(&self) -> &U {
        if self.epoch() < self.cell.epoch() {
            self.epoch.set(self.cell.epoch());
            self.val.set((self.compute)(self.cell.value()));
        }
        &self.val.get().clone()
    }

    fn epoch(&self) -> usize {
        self.epoch.get()
    }
}
