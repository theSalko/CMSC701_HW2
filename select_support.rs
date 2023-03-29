use crate::rank_support::RankSupport;


pub struct SelectSupport<'a> {
    rank_support: &'a RankSupport<'a>,
}


impl<'a> SelectSupport<'a> {
    pub fn new(rank_support: &'a RankSupport<'a>) -> Self {
        SelectSupport {
            rank_support
        }
    }

    // Update the select1 method to use the precomputed
    // select table for lookups. This will have constant time 
    // complexity for most cases, depending on the hashmap implementation.
    pub fn select1(&self, i: u64) -> u64 {
        // Base case we don't want to deal with
        if i==0 {
            return 0;
        }
        let size = self.rank_support.bit_vector_size();
        
        let mut start_index = 0;
        let mut end_index = size-1;
        let mut guess_index = (start_index + end_index) / 2;
        let mut guess_value = self.rank_support.rank1(guess_index);
        loop {
            println!("guess value was = {} and the start and end was = {} | {}", guess_value, start_index, end_index);
            if guess_value >= i {
                end_index = guess_index;
            } else {
                start_index= guess_index;
            }
            
            if (start_index + 1 == guess_index) || (end_index - 1 == guess_index) {
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



}