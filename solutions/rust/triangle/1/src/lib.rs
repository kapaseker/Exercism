pub struct Triangle(u8);

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if (sides[0] + sides[1] > sides[2]) && (sides[1] + sides[2] > sides[0]) && (sides[0] + sides[2] > sides[1]) {
            if sides[0] == sides[1] && sides[1] == sides[2] {
                Some(Triangle(0))
            } else if sides[0] == sides[1] || sides[1] == sides[2] || sides[0] == sides[2] {
                Some(Triangle(1))
            } else {
                Some(Triangle(2))
            }
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.0 == 0
    }

    pub fn is_isosceles(&self) -> bool {
        self.0 == 1 || self.0 == 0 
    }

    pub fn is_scalene(&self) -> bool {
        self.0 == 2
    }
}
