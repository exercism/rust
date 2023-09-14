pub struct Triangle;

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        todo!("Construct new Triangle from following sides: {sides:?}. Return None if the sides are invalid.");
    }

    pub fn is_equilateral(&self) -> bool {
        todo!("Determine if the Triangle is equilateral.");
    }

    pub fn is_scalene(&self) -> bool {
        todo!("Determine if the Triangle is scalene.");
    }

    pub fn is_isosceles(&self) -> bool {
        todo!("Determine if the Triangle is isosceles.");
    }
}
