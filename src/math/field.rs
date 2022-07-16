use super::field_element::FieldElement;
use super::utils::xgcd;


#[derive(Clone, Copy, Debug)]
pub struct Field {
    pub p: i128
}

impl Field {

    pub fn new(p: i128) -> Field {
        Field{p: p}
    }

    pub fn zero(self) -> FieldElement {
        FieldElement::new(0, self)
    }
    
    pub fn one(self) -> FieldElement {
        FieldElement::new(1, self)
    }

    pub fn multiply(self, left: FieldElement, right: FieldElement) -> FieldElement {
        FieldElement::new((left.value * right.value) % self.p, self)
    }

    pub fn subtract(self, left: FieldElement, right: FieldElement) -> FieldElement {
        FieldElement::new((left.value - right.value) % self.p, self)
    }

    pub fn add(self, left: FieldElement, right: FieldElement) -> FieldElement {
        FieldElement::new((left.value + right.value) % self.p, self)
    }

    pub fn negative(self, left: FieldElement) -> FieldElement {
        FieldElement::new((self.p - left.value) % self.p, self)
    }

    pub fn inverse(self, left:FieldElement ) -> FieldElement {
        let (a, _, _) = xgcd(left.value, self.p);
        FieldElement::new(a, self)
    }

    pub fn divide(self, left: FieldElement, right: FieldElement) -> FieldElement {
        assert_ne!(right.value, 0, "divide by zero");
        let (a, _, _) = xgcd(right.value, self.p);
        FieldElement::new(left.value * a % self.p, self)
    }

    pub fn main() -> Field {
        Field::new( 1 + 407 * (1 <<119))
    }

    pub fn generator(self) -> FieldElement {
        assert_eq!(self.p, 1 + 407 * (1 <<119), "do not know generator for other fields");
        FieldElement::new(85408008396924667383611388730472331217, self)
    }

    pub fn primitive_nth_root(self, n: i128) -> FieldElement {
        assert_eq!(self.p, 1 + 407 * (1 <<119), "do not know generator for other fields");
        assert!((n <= (1 <<119)) && (n & (n-1) == 0), "wrong n");
        let mut root: FieldElement = FieldElement::new(85408008396924667383611388730472331217, self);
        let mut order: i128 = 1 << 119;
        while order != n {
            root = root ^ 2;
            order = order / 2;
        }
        root
    }

    pub fn sample(self, byte_array: Vec<u8>) -> FieldElement {
        let mut acc: i128 = 0;
        for b in byte_array {
            acc = (acc << 8) ^ (b as i128);
        }
        FieldElement::new(acc % self.p, self)

    }



}