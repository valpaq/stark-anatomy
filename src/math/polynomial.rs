use super::{field_element::FieldElement};
use std::{ops, cmp};

#[derive(Clone, Debug)]
pub struct Polynomial {
    pub coefficients: Vec<FieldElement>
}

impl Polynomial {

    pub fn new(coefficients: Vec<FieldElement>) -> Polynomial {
        Polynomial{ coefficients: coefficients.clone()}
    }

    pub fn degree(self) -> i128 {

        if self.coefficients.len() == 0 {
            return -1;
        }
        let zero: FieldElement = self.coefficients[0].field.zero();
        // if self.coefficients.iter().all(|&item| item == zero) {
        //     return -1;
        // }
        for (x, y) in self.coefficients.iter().rev().enumerate(){
            if *y != zero {
                return x as i128;
            }
        }
        return -1;
    }

    pub fn is_zero(self) -> bool {
        self.degree() == -1
    }

    pub fn leading_coefficient(self) -> FieldElement {
        return self.coefficients[self.clone().degree() as usize];
    }
}

impl ops::Add<Polynomial> for Polynomial {
    type Output = Polynomial;
    fn add(self, other: Polynomial) -> Polynomial {
        if self.clone().degree() == -1 {
            return other;
        }
        else if other.clone().degree() == -1 {
            return self;
        }
        let field_zero: FieldElement = self.coefficients.get(0).unwrap().field.zero();
        let mut maxlen: usize = cmp::max(
            self.coefficients.len(),
            other.coefficients.len(),
        );
        let mut coeffs: Vec<FieldElement> = vec![field_zero; maxlen];

        for i in 0..self.coefficients.len() {
            coeffs[i] = coeffs[i] + self.coefficients[i];
        }

        for i in 0..other.coefficients.len() {
            coeffs[i] = coeffs[i] + other.coefficients[i];
        }

        return Polynomial::new(coeffs);
    }
}

impl ops::Sub<Polynomial> for Polynomial {
    type Output = Polynomial;
    fn sub(self, other: Polynomial) -> Polynomial {
        return self + (-other);
    }
}

impl ops::Neg for Polynomial {
    type Output = Polynomial;
    fn neg(self) -> Polynomial {
        let mut vec: Vec<FieldElement> = Vec::new();
        for i in self.coefficients {
            vec.push(-i);
        }
        return Polynomial::new(vec);
    }
}

impl ops::Mul for Polynomial {
    type Output = Polynomial;
    fn mul(self, other: Polynomial) -> Polynomial {
        if self.coefficients.clone().len() == 0 || other.coefficients.len() == 0 {
            // let coeffs: Vec<FieldElement> = ;
            return Polynomial::new(Vec::<FieldElement>::new());
        } 
        let zero: FieldElement = self.coefficients[0].field.zero();
        let len: usize = self.coefficients.len() + other.coefficients.len() - 1;
        let mut coeffs: Vec<FieldElement> = vec![zero; len];
        for i in 0..self.coefficients.len() {
            if self.coefficients[i].is_zero() {
                continue;
            }
            for j in 0..other.coefficients.len() {
                coeffs[i + j] = coeffs[i + j] + self.coefficients[i] * other.coefficients[j];
            }
        }
        return Polynomial::new(coeffs);
    }
}

impl PartialEq<Polynomial> for Polynomial {
    fn eq(&self, other: &Polynomial) -> bool {
        if self.clone().degree() != other.clone().degree() {
            return false;
        }
        if self.clone().degree() == -1 {
            return true;
        }

        self.coefficients == other.coefficients
    }

    fn ne(&self, other: &Polynomial) -> bool {
        return !(self == other);
    }
}