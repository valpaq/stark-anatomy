use blake2::{Blake2s256, Digest};

pub struct Merkle {}

impl Merkle {

    fn commit_(leafs: &[u8]) -> [u8; 32] {
        assert!(leafs.len() & (leafs.len() - 1) == 0, "length must be power of two");
        if leafs.len() == 1{
            return leafs.try_into().unwrap();
        }
        else {
            return Blake2s256::digest(&Self::concat(&Self::commit_(&leafs[0..leafs.len()/2]), &Self::commit_(&leafs[leafs.len()/2..]))).into()
        }
    }

    fn concat(left: &[u8], right: &[u8]) -> Vec<u8> {
        let mut concatenated: Vec<u8> = (*left).into();
        let mut right_node_clone: Vec<u8> = (*right).into();
        concatenated.append(&mut right_node_clone);
        concatenated
    }

    fn open_(index: usize, leafs: &[u8]) -> Vec<u8> {
        assert!(leafs.len() & (leafs.len() - 1) == 0, "must be power of two");
        assert!((0 <= index) && (index < leafs.len()), "cannot open invalid index");
        if leafs.len() == 2 {
            return vec![leafs[1-index]];
        }
        if index < leafs.len() / 2 {
            let mut recur_vec = Self::open_(index, &leafs[..leafs.len()/2]);
            recur_vec.extend_from_slice(&Self::commit_(&leafs[leafs.len()/2..]));
            return recur_vec;
        }
        let mut recur_vec = Self::open_(index - leafs.len()/2, &leafs[leafs.len()/2..]);
        recur_vec.extend_from_slice(&Self::commit_(&leafs[..leafs.len()/2]));
        return recur_vec;
    }

    fn verify_(root: [u8; 32], index: usize, path: &[&[u8]], leaf: [u8; 32]) -> bool {
        assert!((0 <= index) && (index < (1 << path.len())), "cannot verify invalid index");
        if path.len() == 1 {
            if index == 0 {
                return root == <[u8;32]>::from(Blake2s256::digest(Self::concat(&leaf, path[0])));
            }
            else {
                return root == <[u8;32]>::from(Blake2s256::digest(Self::concat(path[0], &leaf)));
            }
        }
        else {
            if index % 2 == 0 {
                return Self::verify_(root, index >> 1, &path[1..],  Blake2s256::digest(Self::concat(&leaf, path[0])).into());
            }
            else {
                return Self::verify_(root, index >> 1, &path[1..],  Blake2s256::digest(Self::concat(path[0], &leaf)).into());
            }
        }
    }

    // pub fn commit(data_array: &[[u8;32]]) -> [u8; 32] {
    //     return Self::commit_(data_array.iter().map(|x| Blake2s256::digest(x).into()).collect::<Vec<u8>>().try_into().unwrap())
    // }
}