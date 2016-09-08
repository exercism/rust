use std::collections::HashMap;

pub type CellID = usize;
pub type CallbackID = usize;

struct Cell<'a, T: Copy> {
    value: T,
    last_value: T,
    dependents: Vec<CellID>,
    cell_type: CellType<'a, T>,
    callbacks_issued: usize,
    callbacks: HashMap<CallbackID, Box<FnMut(T) -> () + 'a>>,
}

enum CellType<'a, T: Copy> {
    Input,
    Compute(Vec<CellID>, Box<Fn(&[T]) -> T + 'a>),
}

impl <'a, T: Copy> Cell<'a, T> {
    fn new(initial: T, cell_type: CellType<'a, T>) -> Self {
        Cell {
            value: initial,
            last_value: initial,
            dependents: Vec::new(),
            cell_type: cell_type,
            callbacks_issued: 0,
            callbacks: HashMap::new(),
        }
    }
}

pub struct Reactor<'a, T: Copy> {
    cells: Vec<Cell<'a, T>>,
}

impl <'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Reactor{
            cells: Vec::new(),
        }
    }

    pub fn create_input(&mut self, initial: T) -> CellID {
        self.cells.push(Cell::new(initial, CellType::Input));
        self.cells.len() - 1
    }

    pub fn create_compute<F: Fn(&[T]) -> T + 'a>(&mut self, dependencies: &[CellID], compute_func: F) -> Result<CellID, &'static str> {
        let new_id = self.cells.len();
        for &id in dependencies {
            match self.cells.get_mut(id) {
                Some(c) => c.dependents.push(new_id),
                None => return Err("Nonexistent input"),
            }
        }
        let inputs: Vec<_> = dependencies.iter().map(|&id| self.value(id).unwrap()).collect();
        let initial = compute_func(&inputs);
        self.cells.push(Cell::new(initial, CellType::Compute(dependencies.iter().cloned().collect(), Box::new(compute_func))));
        Ok(new_id)
    }

    pub fn value(&self, id: CellID) -> Option<T> {
        self.cells.get(id).map(|c| c.value)
    }

    pub fn set_value(&mut self, id: CellID, new_value: T) -> Result<(), &'static str> {
        match self.cells.get_mut(id) {
            Some(c) => match c.cell_type {
                CellType::Input => {
                    c.value = new_value;
                    Ok(c.dependents.clone())
                },
                CellType::Compute(_, _) => Err("Can't set compute cell value directly"),
            },
            None => Err("Can't set nonexistent cell"),
        }.map(|deps| {
            for &d in deps.iter() {
                self.update_dependent(d);
            }
            // We can only fire callbacks after all dependents have updated.
            // So we can't combine this for loop with the one above!
            for d in deps {
                self.fire_callbacks(d);
            }
        })
    }

    pub fn add_callback<F: FnMut(T) -> () + 'a>(&mut self, id: CellID, callback: F) -> Result<CallbackID, &'static str> {
        match self.cells.get_mut(id) {
            Some(c) => {
                c.callbacks_issued += 1;
                c.callbacks.insert(c.callbacks_issued, Box::new(callback));
                Ok(c.callbacks_issued)
            },
            None => Err("Can't add callback to nonexistent cell"),
        }
    }

    pub fn remove_callback(&mut self, cell: CellID, callback: CallbackID) -> Result<(), &'static str> {
        match self.cells.get_mut(cell) {
            Some(c) => match c.callbacks.remove(&callback) {
                Some(_) => Ok(()),
                None => Err("Can't remove nonexistent callback"),
            },
            None => Err("Can't remove callback from nonexistent cell"),
        }
    }

    fn update_dependent(&mut self, id: CellID) {
        let (new_value, dependents) = {
            // This block limits the scope of the self.cells borrow.
            // This is necessary becaue we borrow it mutably below.
            let (dependencies, f, dependents) = match self.cells.get(id) {
                Some(c) => match c.cell_type {
                    CellType::Input => panic!("Input cell can't be a dependent"),
                    CellType::Compute(ref dependencies, ref f) => (dependencies, f, c.dependents.clone()),
                },
                None => panic!("Cell to update disappeared while querying"),
            };
            let inputs: Vec<_> = dependencies.iter().map(|&id| self.value(id).unwrap()).collect();
            (f(&inputs), dependents)
        };

        match self.cells.get_mut(id) {
            Some(c) => {
                if c.value == new_value {
                    // No change here, we don't need to update our dependents.
                    // (It wouldn't hurt to, but it would be unnecessary work)
                    return;
                }
                c.value = new_value;
            },
            None => panic!("Cell to update disappeared while updating"),
        }

        for d in dependents {
            self.update_dependent(d);
        }
    }

    fn fire_callbacks(&mut self, id: CellID) {
        let dependents = match self.cells.get_mut(id) {
            Some(c) => {
                if c.value == c.last_value {
                    // Value hasn't changed since last callback fire.
                    // We thus shouldn't fire the callbacks.
                    return
                }
                for cb in c.callbacks.values_mut() {
                    cb(c.value);
                }
                c.last_value = c.value;
                c.dependents.clone()
            },
            None => panic!("Callback cell disappeared"),
        };

        for d in dependents {
            self.fire_callbacks(d);
        }
    }
}
