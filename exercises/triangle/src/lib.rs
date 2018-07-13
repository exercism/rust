pub struct Triangle;

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        unimplemented!("Construct new Triangle from following sides: {:?}. Return None if the sides are invalid.", sides);
    }

    pub fn is_equilateral(&self) -> bool {
        unimplemented!("Determine if the Triangle is equilateral.");
    }

    pub fn is_scalene(&self) -> bool {
        unimplemented!("Determine if the Triangle is scalene.");
    }

    pub fn is_isosceles(&self) -> bool {
        unimplemented!("Determine if the Triangle is isosceles.");
    }
}
