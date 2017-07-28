use std::cmp;

#[derive(Eq,PartialEq,Debug)]
pub enum IceCream {
    Chocolate,
    Vanilla,
    Strawberry,
}

impl IceCream {
    /// Convert various string versions of ice cream flavors to a
    /// proper IceCream enum or return Error
    pub fn get_flavor(request: &str) -> Result<IceCream, &'static str> {
        match request {
            "Chocolate" | "chocolate" | "choc" | "c" | "C" => Ok(Chocolate),
            "Strawberry" | "strawberry" | "straw" | "s" | "S" => Ok(Strawberry),
            "Vanilla" | "vanilla" | "v" | "V" | "plain" => Ok(Vanilla),
            _ => Err("Unknown Flavor"),
        }
    }
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
    pub fn buy(&mut self, qty: u32, flavor: IceCream) -> Option<u32> {
        let pos_qty: u32;
        match flavor {
            Chocolate => {
                pos_qty = cmp::min(qty, self.chocolate);
                self.chocolate -= pos_qty;
            },
            Vanilla => {
                pos_qty = cmp::min(qty, self.vanilla);
                self.vanilla -= pos_qty;
            },
            Strawberry => {
                pos_qty = cmp::min(qty, self.strawberry);
                self.strawberry -= pos_qty;
            },
        }
        if pos_qty > 0 {
            Some(pos_qty)
        } else {
            None
        }
    }

    /// Receives a Vector of tuples with string name of flavor and quantity.
    /// Converts string name into flavor and buy ice cream.
    ///
    /// Returns a vector of quantity_ordered if flavor is valid and
    /// ice cream inventory exists or None.
    pub fn process_order(&mut self, orders: Vec<(&str, u32)>) -> Vec<Option<u32>> {
        let mut out: Vec<Option<u32>> = vec![];
        for (name, qty) in orders {
            match IceCream::get_flavor(name) {
                Ok(flavor) => out.push(self.buy(qty, flavor)),
                Err(_) => out.push(None),
            }
        } 
        out
    }
}
