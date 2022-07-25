use blake2::{Blake2s256, Digest};
use crate::proof_stream::ProofStream;
use crate::math::FieldElement;

pub struct Fri {
    pub offset: u128,
    pub omega: u128,
    pub domain_length: u128,
    pub expansion_factor: u128,
    pub num_colinearity_tests: u128
}

impl Fri {

    pub fn new(offset: u128, omega: u128, 
        domain_length: u128, expansion_factor: u128, num_colinearity_tests: u128) -> Self {
        Fri {
            offset,
            omega,
            domain_length,
            expansion_factor,
            num_colinearity_tests
        }
    }

    pub fn num_rounds(&self) -> u128 {
        let mut codeword_length = self.domain_length;
        let mut num_rounds: u128 = 0;
        while (codeword_length > self.expansion_factor) && 
                (4*self.num_colinearity_tests < codeword_length) {
            codeword_length /= 2;
            num_rounds += 1;
        }
        num_rounds
    }

    pub fn eval_domain(&self) -> Vec<u128>{
        (0..self.domain_length)
            .map(|i| self.offset * (self.omega ^ i))
            .collect::<Vec<u128>>()
    }

    pub fn prove(&self, codeword: &[[u8]], proof_stream: &ProofStream) -> {
        assert!(self.domain_length == codeword.len(), "initial codeword length does not match");
        let codewords = Self::commit(codeword, &proof_stream);
        let top_level_indices = Self::sample_indices(proof_stream.prover_fiat_shamir(), 
            codewords[1].len(), codewords.last().len(), self.num_colinearity_tests);
        let mut indices = top_level_indices.clone();
        for i in 0..(codewords.len()-1) {
            indices = indices.iter().map(|index| index % (codewords[i].len()/2)).collect();
            Self::query(&self, codewords[i], codewords[i+1], &indices, &proof_stream);
        }
        top_level_indices

    }

    pub fn commit(&self, codeword, proof_stream, round_index) Vec<Vec<u8>> {
        let one = FieldElement::ONE;
        let two = FieldElement::new(2);
        let mut omega = self.omega;
        let mut offset = self.offset;
        let mut codewords: Vec<Vec<u8>>;
        
        for r in 0..self.num_rounds(){

            let root = Merkle.commit(codeword);
            proof_stream.push(root);

            if r == self.num_rounds() - 1{
                break;
            }
            let alpha = FieldElement::sample(proof_stream.prover_fiat_shamir());

            codewords.append(codeword);

            codeword = (0..codeword.len()/2).iter().map(
                |i| two.inverse() * ( (one + alpha / (offset * (omega^i)) ) 
                * codeword[i] + (one - alpha / (offset * (omega^i)) ) 
                * codeword[len(codeword)/2 + i] )
            ).collect();

            omega = omega ^ 2;
            offset = offset ^ 2;
        }


        proof_stream.push(codeword);

        codewords.append(codeword);

        codewords
    }

    pub fn query(&self, current_codeword, next_codeword, c_indices, proof_stream) -> {
        let a_indices = c_indices.clone();
        let b_indices = c_indices.iter().map(|index| index + current_codeword.len()/2).collect();

        for s in 0..self.num_colinearity_tests {
            proof_stream.push((current_codeword[]))
        }
    }
}