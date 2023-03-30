use std::io::{Read, Write};
use std::fs::File;
use crate::rank_support::RankSupport;
use crate::select_support::SelectSupport;
use crate::bit_vector::BitVector;
use crate::utils::my_log;
use std::cmp;

pub struct SparseArray  {
    bit_vector: BitVector,
    elements: Vec<String>,
    superchunk_data: Vec<u64>,
    chunk_data: Vec<u16>,
    chunk_size: usize,
    superchunk_size: usize,
}








impl SparseArray {
    pub fn create(size: u64) -> Self {
        let size = size as usize;
        // Create bit vector
        let bit_vector = BitVector::new(size as usize);


        // DO RANK INIT VARIABLES


        let log_of_size = my_log(size);

        // The size of a chunk is .5*log(n)
        let chunk_size: usize = log_of_size/2;

        // The size of a global chunk is (logn)^2
        let superchunk_size: usize = 2*chunk_size*log_of_size;

        // The superchunk has ranks of celing of n/(logn)^2 entries
        let mut superchunk_data = vec![0; (size + superchunk_size - 1) / superchunk_size];
        
        // The chunk has the ranks of celing of n/(.5logn) entries
        let mut chunk_data = vec![0; (size + chunk_size - 1) / chunk_size];
        
        








        // Return the sparse Array





        SparseArray {
            bit_vector,
            elements: Vec::new(),
            superchunk_data,
            chunk_data,
            chunk_size,
            superchunk_size,
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


    // // Literally just 
    pub fn finalize(&mut self) {
        
       // COPY OF RANK INITIALIZATION CODE
       let size =  self.bit_vector.size();

       // log of size is ALWAYS EVEN
       let log_of_size = my_log(size);

       // The size of a chunk is .5*log(n)
       let chunk_size: usize = log_of_size/2;

       // The size of a global chunk is (logn)^2
       let superchunk_size: usize = 2*chunk_size*log_of_size;


       let mut rank = 0;
       let mut prev_superchunk_rank = 0;

       for i in 0..size {
           // if it's divisible by size_of_chuck
           // we add the rank to the global chuck rank
           
           // if it's divisible by size of super chunk
           if i % superchunk_size == 0 {
               // We need to update the sperchunk's rank
               self.superchunk_data[i / superchunk_size] = rank;
               // Keep track of the previous superchunk rank
               prev_superchunk_rank = rank;
               // The first entry is 0 for the sub_chuck so we don't need to update anything
           } else if i % chunk_size == 0 {
               // If it's divisible by the chunk_size we need to update the chunk_data rank
               self.chunk_data[i / chunk_size] = (rank - prev_superchunk_rank) as u16;
           }
           
           // Increase the rank by 1 if we see a 1
           if self.bit_vector.get(i) {
               rank += 1;
           }
           
       }
    }

    // This function places a reference to the r-th present 
    // item in the array in the reference elem. It returns true if 
    // there was >= r items in the sparse array and false otherwise.
    // 0 Indexed
    pub fn get_at_rank(&self, r: u64, s: &mut String) -> bool {
        if r < self.elements.len() as u64 {
            *s = self.elements[r as usize].clone();
            return true;
        } 
        return false;
    }

    // This function looks at the r-th index in the sparse bitvector; 
    // if that bit is 1, it fetches the corresponding value and binds 
    // it to the reference elem and returns true, if that bit is a 0,
    // it simply returns false.
    pub fn get_at_index(&self, r: u64, s: &mut String) -> bool {
        // If there is a 1 there 
        if self.bit_vector.get(r.try_into().unwrap()) {
            // take the rank
            let rank = self.get_rank1(r.try_into().unwrap());
            // return whether you were able to put it in
            // may need rank+1
            return self.get_at_rank(rank, s);
        }
        return false;
       
    }

    // This function takes as its argument a rank r and 
    // returns the index in the sparse array where the r-th present element appears
    pub fn get_index_of(&self, r: u64) -> u64 {
        if r < self.elements.len().try_into().unwrap() {
            return self.get_select1(r+1)-1
        }
        return u64::MAX;
    }

    // This function returns the count of present elements (1s in the bit vector)
    // up to and including index r (Note: This is just rank on the bitvector,
    // but it is inclusive rather than exclusive of index r).
    pub fn num_elem_at(&self, idx: u64) -> u64 {

        if idx as usize == self.bit_vector.size()-1 {
            return self.elements.len().try_into().unwrap();
        }
        return self.get_rank1((idx+1).try_into().unwrap());
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












    pub fn get_overhead(&self) -> usize {
        let bit_vector_size = self.bit_vector.memory_usage();
        let elements_size = self.elements.iter().map(|s| s.len()).sum::<usize>();
        let superchunk_data_size = self.superchunk_data.len() * std::mem::size_of::<u64>();
        let chunk_data_size = self.chunk_data.len() * std::mem::size_of::<u16>();
        let sparse_array_size = bit_vector_size + elements_size + superchunk_data_size + chunk_data_size;
        
        sparse_array_size
    }







    // Print contents of sparse array for debugging purposes:
    pub fn print_everything(&self, bv_step_size: usize) {
        self.bit_vector.print_bit_vector(bv_step_size);
        for element in &self.elements {
            println!("{}", element);
        }
        println!("Printing contents of the CHUNKS AND SUPER CHUNKS");
        self.print_chunks_and_super_chunks();
    }






















    // COPIED FROM RANK SUPPORT

    pub fn get_rank1(&self, i:usize) -> u64{
        if i == 0 {
            return 0;
        }
        let size =  self.bit_vector.size();
    
        let superchunk_rank = self.superchunk_data[i / self.superchunk_size];
        let chunk_rank = self.chunk_data[i / self.chunk_size] as u64;
        let base_rank = superchunk_rank + chunk_rank;
    
       
        let mut rank = base_rank;
        
        let chunk_local_position = i % self.chunk_size;
        let chunk_start =  i - chunk_local_position;
        
        let chunk_end = cmp::min(chunk_start + self.chunk_size, size);
    
        let chunk_as_int = self.bit_vector.interpret_as_u64_int(chunk_start, chunk_end);
        rank += self.bit_vector.get_i_th_rank(chunk_as_int, chunk_local_position);
       
        rank

    }

    
    pub fn print_chunks_and_super_chunks(&self) {
        println!("Printing superchunks");

        let size = self.bit_vector.size();
        let num_superchunks = self.superchunk_data.len();
        let num_chunks = self.chunk_data.len();


        for i in 0..num_superchunks{

            let start = i*self.superchunk_size;
            let end = cmp::min((1+i)*self.superchunk_size, size);
            println!("start is = {} and end is = {}", start, end);
            println!("SuperBlock i = {} has offset value={}", i, self.superchunk_data[i]);
            println!("");

        }

        println!("Printing offset values for in between chunks");
        for i in 0..num_chunks{
            let start = i*self.chunk_size;
            let end = cmp::min((1+i)*self.chunk_size, size);
            println!("start is = {} and end is = {}", start, end);
            println!("Block i = {} has offset value={}", i, self.chunk_data[i]);
            println!("");
             
           
        }

    }

    // COPIED FROM SELECT SUPPORT
    
    // Update the select1 method to use the precomputed
    // select table for lookups. This will have constant time 
    // complexity for most cases, depending on the hashmap implementation.
    pub fn get_select1(&self, i: u64) -> u64 {
        // Base case we don't want to deal with
        if i==0 {
            return 0;
        }
        let size = self.bit_vector.size();
        
        let mut start_index = 0;
        let mut end_index = size-1;
        let mut guess_index = (start_index + end_index) / 2;
        let mut guess_value = self.get_rank1(guess_index);
        loop {
            // println!("guess value was = {} and the start and end was = {} | {}", guess_value, start_index, end_index);
            if guess_value >= i {
                end_index = guess_index;
            } else {
                start_index= guess_index;
            }
            
            if (start_index + 1 == guess_index) || (end_index - 1 == guess_index) {
                assert!(self.get_rank1(guess_index-1) != i);
                break;
            }
            guess_index = (start_index + end_index) / 2;
            guess_value = self.get_rank1(guess_index);
        }
        return guess_index as u64;
        
    }

}












































// pub struct SparseArray<'a> {
//     bit_vector: BitVector,
//     elements: Vec<String>,
//     rank_support: RankSupport<'a>,
//     select_support: SelectSupport<'a>,
// }

// impl SparseArray {
//     // pub fn create(size: u64) -> Self {
//     //     let bit_vector = BitVector::new(size as usize);
//     //     let rank_support = RankSupport::new(&bit_vector);
//     //     let select_support = SelectSupport::new(&rank_support);

//     //     SparseArray {
//     //         // bit_vector,
//     //         // rank_support,
//     //         select_support,
//     //         elements: Vec::new(),
//     //     }
//     // }
//     // pub fn create(size: u64) -> Self {
//     //     let bit_vector = BitVector::new(size as usize);
//     //     let rank_support = RankSupport::new(&bit_vector);
//     //     let select_support = SelectSupport::new(&rank_support);
//     //     // let elements = Vec::new();

//     //     SparseArray {
//     //         bit_vector,
//     //         rank_support,
//     //         select_support,
//     //         elements: Vec::new(),
            
//     //     }
//     // }



//     // Assuming the elements are inserted in order
//     // Appends the element to the end of the list
//     pub fn append(&mut self, elem: String, pos: u64) {

//         // Makes a double check that the position is less 
//         // than the size just to be sure
//         if pos < self.bit_vector.size().try_into().unwrap() {
//             self.bit_vector.set(pos as usize, true);
//             self.elements.push(elem);
//         }
//     }


//     // Literally just 
//     pub fn finalize(&mut self) {
        
//         self.rank_support.finalize_rank();

//     }


//     pub fn get_at_rank(&self, r: u64, s: &mut String) -> bool {
//         if r < self.elements.len() as u64 {
//             *s = self.elements[r as usize].clone();
//             return true;
//         } 
//         return false;
//     }


//     pub fn get_at_index(&self, r: u64, s: &mut String) -> bool {
//         // If there is a 1 there 
//         if self.bit_vector.get(r.try_into().unwrap()) {
//             // take the rank
//             let rank = self.rank_support.rank1(r.try_into().unwrap());
//             // return whether you were able to put it in
//             // may need rank+1
//             return self.get_at_rank(rank, s);
//         }
//         return false;
       
//     }

//     // This function takes as its argument a rank r and 
//     // returns the index in the sparse array where the r-th present element appears
//     pub fn get_index_of(&self, r: u64) -> u64 {
//         self.select_support.select1(r)
//     }

//     // This function returns the count of present elements (1s in the bit vector)
//     // up to and including index r (Note: This is just rank on the bitvector,
//     // but it is inclusive rather than exclusive of index r).
//     pub fn num_elem_at(&self, idx: u64) -> u64 {
//         self.rank_support.rank1((idx+1).try_into().unwrap())
//     }

//     // Returns the size of the sparse array.
//     pub fn size(&self) -> u64 {
//         self.bit_vector.size().try_into().unwrap()
//     }

//     // Returns the number of present elements 
//     // in the sparse array (i.e. the number of 1s in the bitvector).
//     pub fn num_elem(&self) -> u64 {
//         self.elements.len() as u64
//     }


//     pub fn print_something(&self) {
//         println!("lala");
//     }
// }