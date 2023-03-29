use std::io::{Read, Write};
use std::fs::File;
use crate::rank_support::RankSupport;
use crate::select_support::SelectSupport;
use crate::bit_vector::BitVector;

pub struct SparseArray<'a>  {
    bit_vector: BitVector,
    elements: Vec<String>,
    rank_support: RankSupport<'a>,
    select_support: SelectSupport<'a>,
}

impl<'a> SparseArray<'a> {
    pub fn create(size: u64) -> Self {
        let bit_vector = BitVector::new(size as usize);
        let rank_support = RankSupport::new(&bit_vector);
        let select_support = SelectSupport::new(&rank_support);
        // let elements = Vec::new();

        SparseArray {
            bit_vector,
            rank_support,
            select_support,
            elements: Vec::new(),
            
        }
    }


    // Assuming the elements are inserted in order
    // Appends the element to the end of the list
    pub fn append(&mut self, elem: String, pos: u64) {

        // Makes a double check that the position is less 
        // than the size just to be sure
        if pos < self.bit_vector.size().try_into().unwrap() {
            self.bit_vector.set(pos as usize, true);
            self.elements.push(elem);
        }
    }


    // Literally just 
    pub fn finalize(&mut self) {
        
        self.rank_support.finalize_rank();

    }


    pub fn get_at_rank(&self, r: u64, s: &mut String) -> bool {
        if r < self.elements.len() as u64 {
            *s = self.elements[r as usize].clone();
            return true;
        } 
        return false;
    }


    pub fn get_at_index(&self, r: u64, s: &mut String) -> bool {
        // If there is a 1 there 
        if self.bit_vector.get(r.try_into().unwrap()) {
            // take the rank
            let rank = self.rank_support.rank1(r.try_into().unwrap());
            // return whether you were able to put it in
            // may need rank+1
            return self.get_at_rank(rank, s);
        }
        return false;
       
    }

    // This function takes as its argument a rank r and 
    // returns the index in the sparse array where the r-th present element appears
    pub fn get_index_of(&self, r: u64) -> u64 {
        self.select_support.select1(r)
    }

    // This function returns the count of present elements (1s in the bit vector)
    // up to and including index r (Note: This is just rank on the bitvector,
    // but it is inclusive rather than exclusive of index r).
    pub fn num_elem_at(&self, idx: u64) -> u64 {
        self.rank_support.rank1((idx+1).try_into().unwrap())
    }

    // Returns the size of the sparse array.
    pub fn size(&self) -> u64 {
        self.bit_vector.size().try_into().unwrap()
    }

    // Returns the number of present elements 
    // in the sparse array (i.e. the number of 1s in the bitvector).
    pub fn num_elem(&self) -> u64 {
        self.elements.len() as u64
    }


}