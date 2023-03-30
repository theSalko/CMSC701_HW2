use crate::rank_support::RankSupport;
use crate::bit_vector::BitVector;
use std::fs::File;
use std::io::{Read, Write};

pub struct SelectSupport<'a> {
    rank_support: &'a RankSupport<'a>,
}


impl<'a> SelectSupport<'a> {
    pub fn new(rank_support: &'a RankSupport<'a>) -> Self {
        SelectSupport {
            rank_support
        }
    }

    

    // Gives position, in the underlying bit-vector, of the FIRST index, j for which rank1(j) = i.
    pub fn select1(&self, i: u64) -> u64 {
        // Base case we don't want to deal with
        if i==0 {
            return 0;
        }

        let size = self.rank_support.bit_vector_size();
        let total_num_bits_minus_last = self.rank_support.rank1(size-1);
        if i > total_num_bits_minus_last {
            return u64::MAX;
        }
        if i == total_num_bits_minus_last {
            return (size-1) as u64;
        }

        let mut start_index = 0;
        let mut end_index = size-1;
        let mut guess_index = (start_index + end_index) / 2;
        let mut guess_value = self.rank_support.rank1(guess_index);
        loop {
            // println!("guess value was = {} and the start and end was = {} | {}", guess_value, start_index, end_index);
            if guess_value >= i {
                end_index = guess_index;
            } else {
                start_index= guess_index;
            }
            
            if (start_index + 1 == guess_index) || (end_index - 1 == guess_index) {
                if (end_index - 1 == guess_index) {
                    guess_index += 1;
                }
                assert!(self.rank_support.rank1(guess_index) == i);
                assert!(self.rank_support.rank1(guess_index-1) != i);
                break;
            }
            guess_index = (start_index + end_index) / 2;
            guess_value = self.rank_support.rank1(guess_index);
        }
        return guess_index as u64;
        
    }

    // Keep the overhead method as is, but note that the 
    // overhead will be higher due to the select table.
    pub fn overhead(&self) -> u64 {
        self.rank_support.overhead().try_into().unwrap() // + additional overhead for the select_table
    }


    // Saves the select data structure to the file 'fname'.
    pub fn save(&self, fname: &str) -> std::io::Result<()> {
        // Save the rank_support 
        self.rank_support.save(fname)?;

        Ok(())

    }

    // Can only load it given a rank support because it has no other data
    pub fn load(rank_support: &'a RankSupport) -> std::io::Result<Self> {
        
        Ok( Self {
            rank_support: rank_support
        })
    }
}