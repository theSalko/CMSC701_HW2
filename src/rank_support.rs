use crate::bit_vector::BitVector;
use crate::utils::my_log;
use std::fs::File;
use std::io::{Read, Write};
use std::cmp;


pub struct RankSupport<'a> {
    bit_vector: &'a BitVector,
    superchunk_data: Vec<u64>,
    chunk_data: Vec<u16>,
    chunk_size: usize,
    superchunk_size: usize,
}

impl<'a> RankSupport<'a> {

    // Only way I knew how to implement the sparse_array finalize
    pub fn finalize_rank(&mut self) {
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

    pub fn new(bit_vector: &'a BitVector) -> Self {

        let size =  bit_vector.size();
        let log_of_size = my_log(size);

        // The size of a chunk is .5*log(n)
        let chunk_size: usize = log_of_size/2;

        // The size of a global chunk is (logn)^2
        let superchunk_size: usize = 2*chunk_size*log_of_size;

        // The superchunk has ranks of celing of n/(logn)^2 entries
        let mut superchunk_data = vec![0; (size + superchunk_size - 1) / superchunk_size];
        
        // The chunk has the ranks of celing of n/(.5logn) entries
        let mut chunk_data = vec![0; (size + chunk_size - 1) / chunk_size];
        
        

        let mut rank = 0;
        let mut prev_superchunk_rank = 0;

        for i in 0..size {
            // if it's divisible by size_of_chuck
            // we add the rank to the global chuck rank
            
            // if it's divisible by size of super chunk
            if i % superchunk_size == 0 {
                // We need to update the sperchunk's rank
                superchunk_data[i / superchunk_size] = rank;
                // Keep track of the previous superchunk rank
                prev_superchunk_rank = rank;
                // The first entry is 0 for the sub_chuck so we don't need to update anything
            } else if i % chunk_size == 0 {
                // If it's divisible by the chunk_size we need to update the chunk_data rank
                chunk_data[i / chunk_size] = (rank - prev_superchunk_rank) as u16;
            }
            
            // Increase the rank by 1 if we see a 1
            if bit_vector.get(i) {
                rank += 1;
            }
            
        }

        Self {
            bit_vector,
            superchunk_data,
            chunk_data,
            chunk_size,
            superchunk_size,
        }
    }

    pub fn bit_vector_size(&self) -> usize {
        self.bit_vector.size()
    }

    pub fn rank1(&self, i: usize) -> u64 {
    
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

    pub fn overhead(&self) -> usize {
        
        let superchunk_data_bits = self.superchunk_data.len() * 64;
        let chunk_data_bits = self.chunk_data.len() * 16;
        superchunk_data_bits + chunk_data_bits
    }

    pub fn save(&self, file_name: &str) -> std::io::Result<()>  { 

        let mut file = File::create(file_name)?;

        // Save chunk_size and superchunk_size
        file.write_all(&(self.chunk_size as u64).to_le_bytes())?;
        file.write_all(&(self.superchunk_size as u64).to_le_bytes())?;

        
        // Save superchunk_data
        for &value in &self.superchunk_data {
            file.write_all(&value.to_le_bytes())?;
        }

        // Save chunk_data
        for &value in &self.chunk_data {
            file.write_all(&value.to_le_bytes())?;
        }

        Ok(())

    }

    pub fn load(bit_vector: &'a BitVector, file_name: &str) -> std::io::Result<Self> {

        let mut file = File::open(file_name)?;
        
        // Load chunk_size and superchunk_size
        let mut buf = [0u8; 8];
        file.read_exact(&mut buf)?;
        let chunk_size = u64::from_le_bytes(buf) as usize;
        file.read_exact(&mut buf)?;
        let superchunk_size = u64::from_le_bytes(buf) as usize;


        
        // Load superchunk_data
        let mut superchunk_data = Vec::new();
        while file.read_exact(&mut buf).is_ok() {
            superchunk_data.push(u64::from_le_bytes(buf));
        }

        // Load chunk_data
        let mut chunk_data = Vec::new();
        let mut chunk_buf = [0u8; 2];
        while file.read_exact(&mut chunk_buf).is_ok() {
            chunk_data.push(u16::from_le_bytes(chunk_buf));
        }

        Ok(Self {
            bit_vector,
            superchunk_data,
            chunk_data,
            chunk_size,
            superchunk_size,
        })
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


}