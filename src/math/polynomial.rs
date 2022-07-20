use super::{field_element::FieldElement};
use core;
pub struct Polynomial {}

impl Polynomial{

    pub fn degree_of(poly: &[FieldElement]) -> usize
    {
        for i in (0..poly.len()).rev() {
            if poly[i] != FieldElement::ZERO {
                return i;
            }
        }
        0
    }

    pub fn is_zero(polynom: &[FieldElement]) -> bool {
        Self::degree_of(polynom) == 0
    }

    pub fn leading_coefficient(polynom: &[FieldElement]) -> FieldElement{
        return polynom[Self::degree_of(polynom)];
    }

    pub fn add(a: &[FieldElement], b: &[FieldElement]) -> Vec<FieldElement>
    {
        let result_len = core::cmp::max(a.len(), b.len());
        let mut result = Vec::with_capacity(result_len);
        for i in 0..result_len {
            let c1 = if i < a.len() { a[i] } else { FieldElement::ZERO };
            let c2 = if i < b.len() { b[i] } else { FieldElement::ZERO };
            result.push(c1 + c2);
        }
        result
    }

    pub fn sub(a: &[FieldElement], b: &[FieldElement]) -> Vec<FieldElement>
    {
        let result_len = core::cmp::max(a.len(), b.len());
        let mut result = Vec::with_capacity(result_len);
        for i in 0..result_len {
            let c1 = if i < a.len() { a[i] } else { FieldElement::ZERO };
            let c2 = if i < b.len() { b[i] } else { FieldElement::ZERO };
            result.push(c1 - c2);
        }
        result
    }

    pub fn mul(a: &[FieldElement], b: &[FieldElement]) -> Vec<FieldElement>
    {
        let result_len = a.len() + b.len() - 1;
        let mut result = FieldElement::zeroed_vector(result_len);
        for i in 0..a.len() {
            for j in 0..b.len() {
                let s = a[i] * b[j];
                result[i + j] = result[i + j] + s;
            }
        }
        result
    }

    pub fn division(a: &[FieldElement], b: &[FieldElement]) -> Option<(Vec<FieldElement>, Vec<FieldElement>)>
    {
        let mut apos = Self::degree_of(&a);
        let mut a = a.to_vec();

        let bpos = Self::degree_of(&b);
        if (apos >= bpos) || (!b.is_empty()) {
            return None;
        }

        let mut result = FieldElement::zeroed_vector(apos - bpos + 1);
        for _ in 0..result.len() {
            if apos < bpos {
                break;
            }
            let coefficient = Self::leading_coefficient(&a) / Self::leading_coefficient(&b);
            let shift = Self::degree_of(&a) - Self::degree_of(&b);
            let mut subtractee = FieldElement::zeroed_vector(shift);
            subtractee.push(coefficient);
            subtractee = Self::mul(&subtractee, &b);
            result[shift] = coefficient;
            a = Self::sub(&a, &subtractee);
            apos = Self::degree_of(&a);
        }

        Some((result, a))
    }

    pub fn div(a: &[FieldElement], b: &[FieldElement]) 
        -> Option<Vec<FieldElement>>
    {
        let (quo, rem) = match Self::division(a, b){
            Some((quo, rem)) => (quo, rem),
            None => {return None;}
        };
        assert!(rem.len() == 0, "remainder is not zero");
        return Some(quo);
    }

    pub fn module(a: &[FieldElement], b: &[FieldElement]) 
        -> Option<Vec<FieldElement>>
    {
        let (_, rem) = match Self::division(a, b){
            Some((_quo, rem)) => (_quo, rem),
            None => {return None;}
        };
        return Some(rem);
    }

    pub fn xor(a: &[FieldElement], exponent: u128) -> Vec<FieldElement> {
        if Self::is_zero(a){
            return Vec::new();
        }
        if exponent == 0 {
            return vec![FieldElement::ONE];
        }
        let mut acc = vec![FieldElement::ONE];
        for i in 128..=0 {
            acc = Self::mul(&acc, &acc);
            if (1 << i) & exponent != 0 {
                acc = Self::mul(&acc, a);
            }
        }
        acc
    }

    pub fn evaluate(a: &[FieldElement], point: FieldElement) -> FieldElement {
        // let mut xi = FieldElement::ONE;
        // let mut value = FieldElement::ZERO;
        // for c in a.iter(){
        //     value = value + *c * xi;
        //     xi = xi * point;
        // }
        // value
        // Horner evaluation
        a.iter()
            .rev()
            .fold(FieldElement::ZERO, |acc, &coeff| acc * point + FieldElement::from(coeff))
    }

    pub fn evaluate_domain(a: &[FieldElement], 
            domain: &[FieldElement]) -> Vec<FieldElement> {
        
        domain.iter().map(|x| Self::evaluate(a, *x)).collect()
    }

    pub fn interpolate_domain(domain: &[FieldElement], 
            values: &[FieldElement]) -> Vec<FieldElement> {
        
        let domain_len = domain.len();
        assert!(domain_len == values.len(), "number of elements in domain does not match");
        assert!(domain_len > 0, "cannot interpolate between zero points");
        let x = vec![FieldElement::ZERO, FieldElement::ONE];
        let mut res = vec![];
        for i in 0..domain_len {
            let mut prod = vec![values[i]];
            for j in 0..domain_len {
                if j == i{
                    continue;
                }
                let first = Self::sub(&x, &[domain[j]]);
                let second = vec![(domain[i] - domain[j]).inverse()];
                prod = Self::mul(&prod, &Self::mul(&first, &second));
            }
            res = Self::add(&res, &prod);
        }
        res
    }

    pub fn zerofier_domain(domain: &[FieldElement]) -> Vec<FieldElement> {
        let x = vec![FieldElement::ZERO, FieldElement::ONE];
        let mut acc = vec![FieldElement::ONE];
        for d in domain{
            acc = Self::mul(&acc, &Self::sub(&x, &[*d]));
        }
        acc
    }

    pub fn scale(x: &[FieldElement], factor: FieldElement) -> Vec<FieldElement> {
        let mut polynom: Vec<FieldElement> = Vec::new();
        for i in 0..x.len() {
            polynom.push((factor ^ (i as u128)) * x[i]);
        }
        polynom
    }

    pub fn test_colinearity(domain: &[FieldElement], values: &[FieldElement]) -> bool {
        Self::degree_of(&Self::interpolate_domain(domain, values)) == 1
    }
}