use std::collections::HashMap;

/// `InputCellID` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InputCellID(usize);
/// `ComputeCellID` is a unique identifier for a compute cell.
/// Values of type `InputCellID` and `ComputeCellID` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellID = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellID = r.create_compute(&[react::CellID::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComputeCellID(usize);
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CallbackID(usize);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellID {
    Input(InputCellID),
    Compute(ComputeCellID),
}

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

struct Cell<T: Copy> {
    value: T,
    last_value: T,
    dependents: Vec<ComputeCellID>,
}

struct ComputeCell<'a, T: Copy> {
    cell: Cell<T>,

    dependencies: Vec<CellID>,
    f: Box<dyn 'a + Fn(&[T]) -> T>,
    callbacks_issued: usize,
    callbacks: HashMap<CallbackID, Box<dyn 'a + FnMut(T)>>,
}

impl<T: Copy> Cell<T> {
    fn new(initial: T) -> Self {
        Cell {
            value: initial,
            last_value: initial,
            dependents: Vec::new(),
        }
    }
}

impl<'a, T: Copy> ComputeCell<'a, T> {
    fn new<F: 'a + Fn(&[T]) -> T>(initial: T, dependencies: Vec<CellID>, f: F) -> Self {
        ComputeCell {
            cell: Cell::new(initial),

            dependencies,
            f: Box::new(f),
            callbacks_issued: 0,
            callbacks: HashMap::new(),
        }
    }
}

pub struct Reactor<'a, T: Copy> {
    inputs: Vec<Cell<T>>,
    computes: Vec<ComputeCell<'a, T>>,
}

impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Reactor {
            inputs: Vec::new(),
            computes: Vec::new(),
        }
    }

    pub fn create_input(&mut self, initial: T) -> InputCellID {
        self.inputs.push(Cell::new(initial));
        InputCellID(self.inputs.len() - 1)
    }

    pub fn create_compute<F: 'a + Fn(&[T]) -> T>(
        &mut self,
        dependencies: &[CellID],
        compute_func: F,
    ) -> Result<ComputeCellID, CellID> {
        // Check all dependencies' validity before modifying any of them,
        // so that we don't perform an incorrect partial write.
        for &dep in dependencies {
            match dep {
                CellID::Input(InputCellID(id)) => {
                    if id >= self.inputs.len() {
                        return Err(dep);
                    }
                }
                CellID::Compute(ComputeCellID(id)) => {
                    if id >= self.computes.len() {
                        return Err(dep);
                    }
                }
            }
        }
        let new_id = ComputeCellID(self.computes.len());
        for &dep in dependencies {
            match dep {
                CellID::Input(InputCellID(id)) => self.inputs[id].dependents.push(new_id),
                CellID::Compute(ComputeCellID(id)) => {
                    self.computes[id].cell.dependents.push(new_id)
                }
            }
        }
        let inputs: Vec<_> = dependencies
            .iter()
            .map(|&id| self.value(id).unwrap())
            .collect();
        let initial = compute_func(&inputs);
        self.computes.push(ComputeCell::new(
            initial,
            dependencies.to_vec(),
            compute_func,
        ));
        Ok(new_id)
    }

    pub fn value(&self, id: CellID) -> Option<T> {
        match id {
            CellID::Input(InputCellID(id)) => self.inputs.get(id).map(|c| c.value),
            CellID::Compute(ComputeCellID(id)) => self.computes.get(id).map(|c| c.cell.value),
        }
    }

    pub fn set_value(&mut self, id: InputCellID, new_value: T) -> bool {
        let InputCellID(id) = id;
        self.inputs
            .get_mut(id)
            .map(|c| {
                c.value = new_value;
                c.dependents.clone()
            })
            .map(|deps| {
                for &d in deps.iter() {
                    self.update_dependent(d);
                }
                // We can only fire callbacks after all dependents have updated.
                // So we can't combine this for loop with the one above!
                for d in deps {
                    self.fire_callbacks(d);
                }
            })
            .is_some()
    }

    pub fn add_callback<F: 'a + FnMut(T)>(
        &mut self,
        id: ComputeCellID,
        callback: F,
    ) -> Option<CallbackID> {
        let ComputeCellID(id) = id;
        self.computes.get_mut(id).map(|c| {
            c.callbacks_issued += 1;
            let cbid = CallbackID(c.callbacks_issued);
            c.callbacks.insert(cbid, Box::new(callback));
            cbid
        })
    }

    pub fn remove_callback(
        &mut self,
        cell: ComputeCellID,
        callback: CallbackID,
    ) -> Result<(), RemoveCallbackError> {
        let ComputeCellID(cell) = cell;
        match self.computes.get_mut(cell) {
            Some(c) => match c.callbacks.remove(&callback) {
                Some(_) => Ok(()),
                None => Err(RemoveCallbackError::NonexistentCallback),
            },
            None => Err(RemoveCallbackError::NonexistentCell),
        }
    }

    fn update_dependent(&mut self, id: ComputeCellID) {
        let ComputeCellID(id) = id;

        let (new_value, dependents) = {
            // This block limits the scope of the self.cells borrow.
            // This is necessary because we borrow it mutably below.
            let (dependencies, f, dependents) = match self.computes.get(id) {
                Some(c) => (&c.dependencies, &c.f, c.cell.dependents.clone()),
                None => panic!("Cell to update disappeared while querying"),
            };
            let inputs: Vec<_> = dependencies
                .iter()
                .map(|&id| self.value(id).unwrap())
                .collect();
            (f(&inputs), dependents)
        };

        match self.computes.get_mut(id) {
            Some(c) => {
                if c.cell.value == new_value {
                    // No change here, we don't need to update our dependents.
                    // (It wouldn't hurt to, but it would be unnecessary work)
                    return;
                }
                c.cell.value = new_value;
            }
            None => panic!("Cell to update disappeared while updating"),
        }

        for d in dependents {
            self.update_dependent(d);
        }
    }

    fn fire_callbacks(&mut self, id: ComputeCellID) {
        let ComputeCellID(id) = id;
        let dependents = match self.computes.get_mut(id) {
            Some(c) => {
                if c.cell.value == c.cell.last_value {
                    // Value hasn't changed since last callback fire.
                    // We thus shouldn't fire the callbacks.
                    return;
                }
                for cb in c.callbacks.values_mut() {
                    cb(c.cell.value);
                }
                c.cell.last_value = c.cell.value;
                c.cell.dependents.clone()
            }
            None => panic!("Callback cell disappeared"),
        };

        for d in dependents {
            self.fire_callbacks(d);
        }
    }
}
