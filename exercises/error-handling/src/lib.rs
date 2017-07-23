use std::cmp;

#[derive(Eq,PartialEq,Debug)]
pub enum IceCream {
    Chocolate,
    Vanilla,
    Strawberry,
}

#[derive(Debug)]
pub struct IceCreamTruck {
    // These fields are only pub, because tests are not in the same file
    // and we need to verify inventory after actions.
    pub chocolate: u32,
    pub vanilla: u32,
    pub strawberry: u32,
}

use IceCream::*;

impl IceCreamTruck {
    pub fn new(chocolate: u32, vanilla: u32, strawberry: u32) -> Self {
        IceCreamTruck { chocolate: chocolate,
                        vanilla: vanilla,
                        strawberry: strawberry }
    }

    /// Buy ice cream and update inventory.
    /// Return quantity purchased or indication that you could not purchase.
    pub fn buy() {
    }

    /// Receives a Vector of tuples with string name of flavor and quantity.
    /// Converts string name into flavor and buy ice cream.
    ///
    /// Returns a vector of quantity_ordered if flavor is valid and
    /// ice cream inventory exists or None.
    pub fn process_order() {
    }
}
