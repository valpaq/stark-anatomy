use sha3::{Sha3_256, Digest};
use std::slice;
use crate::math::errors::DeserializationError;

pub struct ProofStream{
    pub objects: Vec<Vec<u8>>,
    pub read_index: usize
}

impl ProofStream {

    pub fn new() -> Self {
        ProofStream {
            objects: Vec::<Vec<u8>>::new(),
            read_index: 0
        }
    }

    pub fn push(mut self, obj: Vec<u8>) {
        self.objects.push(obj);
    }

    pub fn pull(mut self) -> Vec<u8> {
        assert!(self.read_index < self.objects.len(), "ProofStream: queue empty");
        let obj = &self.objects[self.read_index];
        self.read_index += 1;
        return obj.to_vec();
    }

    fn vf_to_u8(v: &[Vec<u8>]) -> &[u8] {
        unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * 4) }
    }

    pub fn prover_fiat_shamir(&self) -> [u8; 32] {
        Sha3_256::digest(&mut Self::vf_to_u8(&self.objects)).into()
    }

    pub fn verifier_fiat_shamir(&self) -> [u8; 32] {
        Sha3_256::digest(&mut Self::vf_to_u8(&self.objects[0..self.read_index])).into()
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Challenge([u8; 32]);

impl AsRef<[u8]> for Challenge {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<[u8; 32]> for Challenge {
    fn from(id: [u8; 32]) -> Self {
        Self(id)
    }
}
