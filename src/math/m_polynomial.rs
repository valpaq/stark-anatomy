use std::collections::HashMap;
use super::field_element::FieldElement;
use super::polynomial::Polynomial;
use std::cmp;


#[derive(Clone, Debug)]
pub struct MPolynomial {}

impl MPolynomial {

    // pub fn new(map: &HashMap<Vec<u128>, FieldElement>) -> Self {
    //     MPolynomial{
    //         map: map.clone()
    //     }
    // }

    pub fn zero() -> HashMap<Vec<u128>, FieldElement> {
        HashMap::new()
    }

    pub fn num_variables(left: &HashMap<Vec<u128>, FieldElement>, 
        right: &HashMap<Vec<u128>, FieldElement>) -> usize {
        
        cmp::max(left.keys().map(|x| x.len()).max().unwrap_or(0), right.keys().map(|x| x.len()).max().unwrap_or(0))
        
        
    }

    pub fn add(left: &HashMap<Vec<u128>, FieldElement>, 
        right: &HashMap<Vec<u128>, FieldElement>) 
            -> HashMap<Vec<u128>, FieldElement> 
            {
    
        let mut res: HashMap<Vec<u128>, FieldElement> = HashMap::new();
        let num_variables = Self::num_variables(&left, &right);
        for (k, v) in left.iter() {
            let mut pad = k.to_vec();
            pad.append(&mut vec![0; num_variables - k.len()]);
            res.insert(pad, *v);
        }
        for (k, v) in right.iter() {
            let mut pad = k.to_vec();
            pad.append(&mut vec![0; num_variables - k.len()]);
            res.insert(pad.clone(), *v + if res.contains_key(&pad) 
                { res[&pad] } else { FieldElement::new(0) });
        }     
        res 
    }

    pub fn mul(left: &HashMap<Vec<u128>, FieldElement>, 
        right: &HashMap<Vec<u128>, FieldElement>) 
            -> HashMap<Vec<u128>, FieldElement> 
            {
    
        let mut res: HashMap<Vec<u128>, FieldElement> = HashMap::new();
        let num_variables = Self::num_variables(&left, &right);
        for (k0, v0) in left.iter() {
            for (k1, v1) in right.iter() {
                let mut exponent = vec![0; num_variables];
                for k in 0..k0.len() {
                    exponent[k] += k0[k]
                }
                for k in 0..k1.len() {
                    exponent[k] += k1[k]
                }
                res.insert(exponent.clone(), 
                    (*v0)*(*v1) + if res.contains_key(&exponent) { res[&exponent] }
                    else { FieldElement::new(0) });                
            }
        }
        res
    }

    pub fn sub(left: &HashMap<Vec<u128>, FieldElement>, 
        right: &HashMap<Vec<u128>, FieldElement>) -> HashMap<Vec<u128>, FieldElement> {
        
        let mut res: HashMap<Vec<u128>, FieldElement> = HashMap::new();
        let num_variables = Self::num_variables(&left, &right);
        for (k, v) in left.iter() {
            let mut pad = k.to_vec();
            pad.append(&mut vec![0; num_variables - k.len()]);
            res.insert(pad, *v);
        }
        for (k, v) in right.iter() {
            let mut pad = k.to_vec();
            pad.append(&mut vec![0; num_variables - k.len()]);
            res.insert(pad.clone(), FieldElement::new(0) - *v + if res.contains_key(&pad) 
                { res[&pad] } else { FieldElement::new(0) });
        }     
        res 
    }

    pub fn xor(left: &HashMap<Vec<u128>, FieldElement>, 
        exponent: usize) -> HashMap<Vec<u128>, FieldElement> {

        
        if Self::is_zero(&left) {
            return HashMap::new();
        }
        let num_variables = left.keys().next().unwrap().len();
        let exp = vec![0; num_variables];
        let mut acc: HashMap<Vec<u128>, FieldElement> = HashMap::new();
        acc.insert(exp, FieldElement::ONE);
        for i in 128..=0 {
            acc = Self::mul(&acc, &acc);
            if (1 << i) & exponent != 0 {
                acc = Self::mul(&acc, &left);
            }
        }
        acc
    }

    pub fn constant(element: FieldElement) -> HashMap<Vec<u128>, FieldElement> {
        let mut map: HashMap<Vec<u128>, FieldElement> = HashMap::new();
        map.insert(vec![0], element);
        map
    }

    pub fn is_zero(left: &HashMap<Vec<u128>, FieldElement>) -> bool {
        for v in left.values() {
            if !FieldElement::is_zero(&v){
                return false;
            }
        }
        return true;
    }

    pub fn variables(num_variables: usize) -> Vec<HashMap<Vec<u128>, FieldElement>> {
        let mut variables: Vec<HashMap<Vec<u128>, FieldElement>> = Vec::new();
        for i in 0..num_variables {
            let mut exponent: Vec<u128> = vec![0; i];
            exponent.append(&mut vec![1]);
            exponent.append(&mut vec![0; num_variables - i - 1]);
            let mut tmp_hash = HashMap::new();
            tmp_hash.insert(exponent, FieldElement::ONE);
            variables.push(tmp_hash);
        }
        variables
    }

    pub fn lift(poly: &[FieldElement], 
        variable_index: usize) -> HashMap<Vec<u128>, FieldElement> {
        
        if Polynomial::is_zero(&poly) {
            return HashMap::new();
        }
        let variables = Self::variables(variable_index +1);
        let x = variables.last().expect("no last on variables");
        let mut acc: HashMap<Vec<u128>, FieldElement> = HashMap::new();
        for i in 0..poly.len() {
            let tmp_hash = Self::mul(&Self::constant(poly[i]), &Self::xor(&x, i));
            acc = Self::add(&acc, &tmp_hash);
        }
        acc
    }

    pub fn evaluate(left: &HashMap<Vec<u128>, FieldElement>, point: Vec<FieldElement>) 
        -> FieldElement {
        let mut acc: FieldElement = FieldElement::ZERO;
        for (k, v) in left.iter() {
            let mut prod = *v;
            for i in 0..k.len(){
                prod = prod * (point[i] ^ k[i]);
            }
            acc = acc + prod;
        }
        acc
    }

    // pub fn evaluate_symbolic(left: &HashMap<Vec<u128>, FieldElement>, point: Vec<FieldElement>) 
    //     -> Vec<FieldElement>{
        
    //     let mut acc: Vec<FieldElement> = Vec::new();
    //     for (k, v) in left.iter() {
    //         let mut prod = vec![*v];
    //         for i in 0..k.len() {
    //             prod = Polynomial::mul(&prod, point[i] ^ k[i]);
    //         }
    //         acc = Polynomial::add(&acc, &prod);
    //     }
    //     acc
    // }


}