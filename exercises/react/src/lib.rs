#[allow(unused_variables)]

// Because these are passed without & to some functions,
// it will probably be necessary for these two types to be Copy.
pub type CellID = ();
pub type CallbackID = ();

pub struct Reactor<T> {
    // Just so that the compiler doesn't complain about an unused type parameter.
    // You probably want to delete this field.
    dummy: T,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl <T: Copy + PartialEq> Reactor<T> {
    pub fn new() -> Self {
        unimplemented!()
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> CellID {
        unimplemented!()
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // Return an Err (and you can change the error type) if any dependency doesn't exist.
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Fn(&[T]) -> T>(&mut self, dependencies: &[CellID], compute_func: F) -> Result<CellID, ()> {
        unimplemented!()
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellID) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellID) -> Option<T> {
        unimplemented!()
    }

    // Sets the value of the specified input cell.
    //
    // Return an Err (and you can change the error type) if the cell does not exist, or the
    // specified cell is a compute cell, since compute cells cannot have their values directly set.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellID) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: CellID, new_value: T) -> Result<(), ()> {
        unimplemented!()
    }

    // Adds a callback to the specified compute cell.
    //
    // Return an Err (and you can change the error type) if the cell does not exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) -> ()>(&mut self, id: CellID, callback: F) -> Result<CallbackID, ()> {
        unimplemented!()
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Return an Err (and you can change the error type) if either the cell or callback
    // does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(&mut self, cell: CellID, callback: CallbackID) -> Result<(), ()> {
        unimplemented!()
    }
}
