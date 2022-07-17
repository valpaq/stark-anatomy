use std::ops::*;
use core::slice;
use core::mem;
use super::{M, ELEMENT_BYTES,
    utils::{add, inv, sub, mul},
    errors::*
};

// FIELD ELEMENT
// ================================================================================================

/// Represents a base field element.
///
/// Internal values are stored in their canonical form in the range [0, M). The backing type is
/// `u128`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct FieldElement(u128);

impl FieldElement {

    pub const ZERO: Self = FieldElement(0);
    pub const ONE: Self = FieldElement(1);
    pub const ELEMENT_BYTES: usize = ELEMENT_BYTES;
    pub const GENERATOR: Self = FieldElement(3);


    pub const fn new(value: u128) -> Self {
        FieldElement(if value < M { value } else { value - M })
    }

    pub fn inverse(&self) -> FieldElement {
        FieldElement(inv(self.0))
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    pub fn zeroed_vector(n: usize) -> Vec<Self> {
        // this uses a specialized vector initialization code which requests zero-filled memory
        // from the OS; unfortunately, this works only for built-in types and we can't use
        // Self::ZERO here as much less efficient initialization procedure will be invoked.
        // We also use u128 to make sure the memory is aligned correctly for our element size.
        let result = vec![0u128; n];

        // translate a zero-filled vector of u128s into a vector of base field elements
        let mut v = core::mem::ManuallyDrop::new(result);
        let p = v.as_mut_ptr();
        let len = v.len();
        let cap = v.capacity();
        unsafe { Vec::from_raw_parts(p as *mut Self, len, cap) }
    }

    pub fn elements_as_bytes(elements: &[Self]) -> &[u8] {
        // TODO: take endianness into account
        let p = elements.as_ptr();
        let len = elements.len() * Self::ELEMENT_BYTES;
        unsafe { slice::from_raw_parts(p as *const u8, len) }
    }

    pub unsafe fn bytes_as_elements(bytes: &[u8]) -> Result<&[Self], DeserializationError> {
        if bytes.len() % Self::ELEMENT_BYTES != 0 {
            return Err(DeserializationError::InvalidValue(format!(
                "number of bytes ({}) does not divide into whole number of field elements",
                bytes.len(),
            )));
        }

        let p = bytes.as_ptr();
        let len = bytes.len() / Self::ELEMENT_BYTES;

        if (p as usize) % mem::align_of::<u128>() != 0 {
            return Err(DeserializationError::InvalidValue(
                "slice memory alignment is not valid for this field element type".to_string(),
            ));
        }

        Ok(slice::from_raw_parts(p as *const Self, len))
    }

    pub fn generator(&self) -> FieldElement {
        FieldElement::new(85408008396924667383611388730472331217)
    }

    pub fn primitive_nth_root(&self, n: u128) -> FieldElement {
        assert!((n <= (1 <<119)) && (n & (n-1) == 0), "wrong n");
        let mut root: FieldElement = FieldElement::new(85408008396924667383611388730472331217);
        let mut order: u128 = 1 << 119;
        while order != n {
            root = root ^ 2;
            order = order / 2;
        }
        root
    }
}

impl Add for FieldElement {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(add(self.0, rhs.0))
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(sub(self.0, rhs.0))
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self(mul(self.0, rhs.0))
    }
}

impl Div for FieldElement {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self(mul(self.0, inv(rhs.0)))
    }
}

impl Neg for FieldElement {
    type Output = Self;

    fn neg(self) -> Self {
        Self(sub(0, self.0))
    }
}

impl BitXor<u128> for FieldElement {
    type Output = Self;
    fn bitxor(self, exponent: u128) -> FieldElement {
        Self(self.0 ^ exponent)
    }
}

impl From<u128> for FieldElement {
    /// Converts a 128-bit value into a field element. If the value is greater than or equal to
    /// the field modulus, modular reduction is silently performed.
    fn from(value: u128) -> Self {
        FieldElement::new(value)
    }
}

