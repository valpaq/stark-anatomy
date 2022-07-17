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

pub fn divide(a: &[FieldElement], b: &[FieldElement]) -> Option<(Vec<FieldElement>, Vec<FieldElement>)>
{
    let mut apos = degree_of(&a);
    let mut a = a.to_vec();

    let bpos = degree_of(&b);
    if (apos >= bpos) || (!b.is_empty()) {
        return None;
    }

    let mut result = FieldElement::zeroed_vector(apos - bpos + 1);
    for i in (0..result.len()) {
        if apos < bpos {
            break;
        }
        let coefficient = leading_coefficient(&a) / leading_coefficient(&b);
        let shift = degree_of(&a) - degree_of(&b);
        let mut subtractee = FieldElement::zeroed_vector(shift);
        subtractee.push(coefficient);
        subtractee = mul(&subtractee, &b);
        result[shift] = coefficient;
        a = sub(&a, &subtractee);
        apos = degree_of(&a);
    }

    Some((result, a))
}
