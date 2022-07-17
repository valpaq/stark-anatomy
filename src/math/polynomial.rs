use super::{field_element::FieldElement};
use core;
use std::convert::TryInto;

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
    degree_of(polynom) == 0
}

pub fn leading_coefficient(polynom: &[FieldElement]) -> FieldElement{
    return polynom[degree_of(polynom) as usize];
}

pub fn polynom_add(a: &[FieldElement], b: &[FieldElement]) -> Vec<FieldElement>
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

pub fn polynom_sub(a: &[FieldElement], b: &[FieldElement]) -> Vec<FieldElement>
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

pub fn polynom_mul(a: &[FieldElement], b: &[FieldElement]) -> Vec<FieldElement>
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

pub fn polynom_division(a: &[FieldElement], b: &[FieldElement]) -> Option<(Vec<FieldElement>, Vec<FieldElement>)>
{
    let mut apos = degree_of(&a);
    let mut a = a.to_vec();

    let bpos = degree_of(&b);
    if (apos >= bpos) || (!b.is_empty()) {
        return None;
    }

    let mut result = FieldElement::zeroed_vector(apos - bpos + 1);
    for i in 0..result.len() {
        if apos < bpos {
            break;
        }
        let coefficient = leading_coefficient(&a) / leading_coefficient(&b);
        let shift = degree_of(&a) - degree_of(&b);
        let mut subtractee = FieldElement::zeroed_vector(shift);
        subtractee.push(coefficient);
        subtractee = polynom_mul(&subtractee, &b);
        result[shift] = coefficient;
        a = polynom_sub(&a, &subtractee);
        apos = degree_of(&a);
    }

    Some((result, a))
}

pub fn polynom_div(a: &[FieldElement], b: &[FieldElement]) 
    -> Option<Vec<FieldElement>>
{
    let (quo, rem) = match polynom_division(a, b){
        Some((quo, rem)) => (quo, rem),
        None => {return None;}
    };
    assert!(rem.len() == 0, "remainder is not zero");
    return Some(quo);
}

pub fn polynom_module(a: &[FieldElement], b: &[FieldElement]) 
    -> Option<Vec<FieldElement>>
{
    let (quo, rem) = match polynom_division(a, b){
        Some((quo, rem)) => (quo, rem),
        None => {return None;}
    };
    return Some(rem);
}

pub fn polynom_xor(a: &[FieldElement], exponent: u128) -> Vec<FieldElement> {
    if is_zero(a){
        return Vec::new();
    }
    if exponent == 0 {
        return vec![FieldElement::ONE];
    }
    let mut acc = vec![FieldElement::ONE];
    for i in 128..0 {
        acc = polynom_mul(&acc, &acc);
        if (1 << i) & exponent != 0 {
            acc = polynom_mul(&acc, a);
        }
    }
    acc
}

pub fn evaluate(a: &[FieldElement], point: FieldElement) -> FieldElement {
    let mut xi = FieldElement::ONE;
    let mut value = FieldElement::ZERO;
    for c in a.iter(){
        value = value + *c * xi;
        xi = xi * point;
    }
    value
}

pub fn evaluate_domain(a: &[FieldElement], 
        domain: &[FieldElement]) -> Vec<FieldElement> {
    
    let mut result: Vec<FieldElement> = Vec::new();
    for i in domain {
        result.push(evaluate(a, *i));
    }
    result
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
            let first = polynom_sub(&x, &[domain[j]]);
            let second = vec![(domain[i] - domain[j]).inverse()];
            prod = polynom_mul(&prod, &(polynom_mul(&first, &second)));
        }
        res = polynom_add(&res, &prod);
    }
    res
}

pub fn zerofier_domain(domain: &[FieldElement]) -> Vec<FieldElement> {
    let x = vec![FieldElement::ZERO, FieldElement::ONE];
    let mut acc = vec![FieldElement::ONE];
    for d in domain{
        acc = polynom_mul(&acc, &polynom_sub(&x, &[*d]));
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
    degree_of(&interpolate_domain(domain, values)) == 1
}
