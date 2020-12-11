use std::collections::BTreeSet;
use super::vector::Vector3;
use num_traits::Float;

pub(crate) fn greedy_meshing<T: Float>(voxels: &BTreeSet<[isize;3]>, step: T){
    let mut boxes = Vec::new(); 
}