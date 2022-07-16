use std::ops;
use super::field::Field;


#[derive(Clone, Copy, Debug)]
pub struct FieldElement {
    pub value: i128,
    pub field: Field,
}

impl FieldElement {
    pub fn new(value: i128, field: Field) -> FieldElement {
        FieldElement {
            value: value,
            field: field,
        }
    }

    pub fn inverse(self) -> FieldElement {
        self.field.inverse(self)
    }

    pub fn is_zero(self) -> bool {
        self.value == 0
    }
}

impl ops::Add<FieldElement> for FieldElement {
    type Output = FieldElement;
    fn add(self, other: FieldElement) -> FieldElement {
        assert_eq!(self.field.p, other.field.p, "different fields");
        self.field.add(self, other)
    }
}

impl ops::Sub<FieldElement> for FieldElement {
    type Output = FieldElement;
    fn sub(self, other: FieldElement) -> FieldElement {
        assert_eq!(self.field.p, other.field.p, "different fields");
        self.field.subtract(self, other)
    }
}

impl ops::Mul<FieldElement> for FieldElement {
    type Output = FieldElement;
    fn mul(self, other: FieldElement) -> FieldElement {
        assert_eq!(self.field.p, other.field.p, "different fields");
        self.field.multiply(self, other)
    }
}

impl ops::Div<FieldElement> for FieldElement {
    type Output = FieldElement;
    fn div(self, other: FieldElement) -> FieldElement {
        assert_eq!(self.field.p, other.field.p, "different fields");
        self.field.divide(self, other)
    }
}

impl ops::Neg for FieldElement {
    type Output = FieldElement;
    fn neg(self) -> FieldElement {
        self.field.negative(self)
    }
}

impl ops::BitXor<i128> for FieldElement {
    type Output = FieldElement;
    fn bitxor(self, exponent: i128) -> FieldElement {
        let mut acc = FieldElement::new(1, self.field);
        let val = FieldElement::new(self.value, self.field);
        for i in (0..format!("{:b}", exponent).chars().count()).rev() {
            acc = acc * acc;
            if (1 << i) & exponent != 0 {
                acc = acc * val;
            }
        }

        acc
    }
}

impl PartialEq<FieldElement> for FieldElement {
    fn eq(&self, other: &FieldElement) -> bool {
        self.value == other.value
    }

    fn ne(&self, other: &FieldElement) -> bool {
        self.value != other.value
    }
}