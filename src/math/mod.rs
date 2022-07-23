mod utils;
mod field_element;
mod polynomial;
pub mod errors;
mod m_polynomial;


// Field modulus = 2^128 - 45 * 2^40 + 1
pub const M: u128 = 270497897142230380135924736767050121217;

// Number of bytes needed to represent field element
pub const ELEMENT_BYTES: usize = core::mem::size_of::<u128>();