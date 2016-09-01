// TODO: For now, lib is symlinked to example to ease local development.
// But the final plan is to provide a stub file once we know what the interface will be.

pub type CellID = usize;

pub struct Cell<'a, T: Copy> {
    val: T,
    dependents: Vec<CellID>,
    cell_type: CellType<'a, T>,
}

pub enum CellType<'a, T: Copy> {
    Input,
    Compute(Vec<CellID>, Box<Fn(&[T]) -> T + 'a>),
}

pub struct Reactor<'a, T: Copy> {
    cells: Vec<Cell<'a, T>>,
}

impl <'a, T: Copy> Reactor<'a, T> {
    pub fn new() -> Self {
        Reactor{
            cells: Vec::new(),
        }
    }

    pub fn create_input(&mut self, initial: T) -> CellID {
        self.cells.push(Cell {
            val: initial,
            dependents: Vec::new(),
            cell_type: CellType::Input,
        });
        self.cells.len() - 1
    }

    pub fn create_compute<F: Fn(&[T]) -> T + 'a>(&mut self, dependencies: &[CellID], compute_func: F) -> CellID {
        let inputs: Vec<_> = dependencies.iter().map(|&id| self.get(id).unwrap().value()).collect();
        let initial = compute_func(&inputs);
        self.cells.push(Cell {
            val: initial,
            dependents: Vec::new(),
            cell_type: CellType::Compute(dependencies.iter().cloned().collect(), Box::new(compute_func)),
        });
        self.cells.len() - 1
    }

    pub fn get(&self, id: CellID) -> Option<&Cell<'a, T>> {
        self.cells.get(id)
    }

    pub fn get_mut(&mut self, id: CellID) -> Option<&mut Cell<'a, T>> {
        self.cells.get_mut(id)
    }
}

impl <'a, T: Copy> Cell<'a, T> {
    pub fn value(&self) -> T {
        self.val
    }

    pub fn set_value(&mut self, new_val: T) {
        match self.cell_type {
            CellType::Input => self.val = new_val,
            CellType::Compute(_, _) => panic!("Can't set compute value directly"),
        }
    }
}
