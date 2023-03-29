
use std::io::{Read, Write};
use std::fs::File;

pub struct BitVector {
    data: Vec<u64>,
    size: usize,
}

impl BitVector {
    
    // Instantiates a new BitVector
    pub fn new(size: usize) -> Self {
        let data = vec![0; (size + 64) / 64];
        Self { data, size }
    }

    // Returns the size of the BitVector
    pub fn size(&self) -> usize{
        self.size
    }

    // Sets the value of the bit at index position with
    // value 
    pub fn set(&mut self, index: usize, value: bool) {

        // make sure that the index is smaller than the size of the BitVector
        assert!(index < self.size, "Index out of bounds");

        // The array index in data is index/64 
        let array_index = index / 64;
        // The bit index is the remainder modulo 64 
        let bit_index = index % 64;

        // If we're trying to change the thing at position bit_index
        // means we'll manipulate the value at array_index 
        //  with 2^(bit_index)
        let mask = 1u64 << bit_index;

        // if we wish to change the bit to 1
        if value {
            // OR the current value with the mask
            self.data[array_index] |= mask;
        } else {
            // if we want to change it to 0
            // we need to AND the zero at 
            // bit_index position and ones everywhere else
            // means we'll NOT the mask and AND
            // with the value at array_index
            self.data[array_index] &= !mask;
        }
    }


    pub fn get(&self, index: usize) -> bool { 
        // Check that the index is within the bounds
        assert!(index < self.size, "Index out of bounds");

        // The array index in data is index/64 
        let array_index = index / 64;
        // The bit index is the remainder modulo 64 
        let bit_index = index % 64;

        // If we're trying to change the thing at position bit_index
        // means we'll manipulate the value at array_index 
        //  with 2^(bit_index)
        let mask = 1u64 << bit_index;

        // Once we AND it with 000010000 if we're
        // left with a 0 it means the bit value was 0
        // if we're left with 2^bit_index then the bit value was 1
        // So if it's not equal to 0 that means we need to return a 1 (true)
        // else we return a 0 (false)
        (self.data[array_index] & mask) != 0
    }


    pub fn save(&self, file_name: &str) -> std::io::Result<()> { 
        // Create the file
        let mut file = File::create(file_name)?;
        // Write the size
        file.write_all(&(self.size as u64).to_le_bytes())?;
        // Write all the values from data
        for &value in &self.data {
            file.write_all(&value.to_le_bytes())?;
        }
        Ok(())
    }

    pub fn load(file_name: &str) -> std::io::Result<Self> {


        // Open the file with file_name
        let mut file = File::open(file_name)?;

        // Initialize a buffer you'll read the size from
        let mut size_buf = [0u8; 8];
        // Read the size into the buffer
        file.read_exact(&mut size_buf)?;
        // Convert it into u64
        let size = u64::from_le_bytes(size_buf) as usize;

        // Initialize the data Vector
        let mut data = Vec::new();
        // Initilize the buffer you'll read the values into
        let mut value_buf = [0u8; 8];
        // While you can read, push the values onto the data
        while file.read_exact(&mut value_buf).is_ok() {
            data.push(u64::from_le_bytes(value_buf));
        }

        // Return an instance of Self with data and size
        Ok(Self { data, size })
        
    }

    // for debugging purposes print the bits of the vector
    pub fn print_bit_vector(&self, step_size: usize) {
        for i in 0..self.size {
            let bit = self.get(i);
            print!("{}", if bit { "1" } else { "0" });
            if (i % step_size == step_size-1) {
                println!(" | {i}");
            }
        }
        println!();
    }

    pub fn get_i_th_rank(&self, number:u64, i:usize) -> u64 {
        if i==0 {
            return 0;
        }
        // If we're looking at rank for i=1 we need only look at the even/odd bit
        if i == 1 {
            return number % 2;
        }

        // The number with the (i-1)-th bit leading
        let i_lead = number << (64-i);
        // Count the number of ones
        let num_ones_before_i = i_lead.count_ones();
        // that's the result
        return num_ones_before_i.into();
    }

    pub fn get_first_x_bits(&self, number: u64, x: usize) -> u64 {
        let mask = (1u64 << x) - 1;
        number & mask
    }

    pub fn get_first_x_bits_from_left(&self, number: u64, x: usize) -> u64 {
        // If you want 64 or more we return the number
        if x >= 64 {
            return number;
        }

        // If you want 0 we give you a 0
        if x == 0 {
            return 0;
        }
    
        let shift_amount = 64 - x;
        let mask = (1u64 << x) - 1;
        (number >> shift_amount) & mask
    }

    pub fn interpret_as_u64_int(&self, i:usize, j:usize) -> u64{
        assert!(j > i);
        assert!(j - i <= 64);
        // println!("passed the assert");
        let i_array_index:usize = i/64;
        let i_array_offset:usize = i % 64;
        let j_array_index:usize = j/64;
        let j_array_offset:usize = j % 64;

        let mut result:u64 = 0;

        if i_array_index == j_array_index {
            // They share the number
            let full_number:u64 = self.data[i_array_index];
            // The mask is 0000111111100000 (2^j - 2^i gives us exclusive with j)
            let mask = (1u64 << j_array_offset) - (1u64 << i_array_offset);
            // And it and give the offset
            result = (full_number & mask) >> i_array_offset;

        } else {
            // for j we want the first j_array_index numbers
            let x = self.data[j_array_index];
            let first_number:u64 = self.get_first_x_bits(self.data[j_array_index], j_array_offset);
            // println!("passed first one {}", first_number);
            // Get the leftmost 64 - i_array_offset bits
            let second_number:u64 = self.get_first_x_bits_from_left(self.data[i_array_index], 64 - i_array_offset);
            // println!("Passed second one: {}", second_number);
            // Result it once you shift the j number by the appropriate number of bits so 
            // we can just add them where they need to be added

            // if the i_array_offset is different than 0 
            if i_array_offset != 0 {
                // We shift the first number by 64 - i_array_offset bits because that's
                // How many we're taking from the i_array_index number
                result = second_number + (first_number << (64 - i_array_offset));
            } else {
                // if it's 0 that means j is 0 and that we're getting the full number from
                // the i_array_index which is just the second number
                result = second_number;
            }
            
        }

        return result;
    }


}