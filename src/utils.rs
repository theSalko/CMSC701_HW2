use crate::bit_vector::BitVector;

use std::io::{Read, Write};
use std::fs::File;


// Returns the log of the number first rounded down to
// the nearest integer and then rounded up to the
// nearest even integer
// 2*\celing{\floor{log(n)}/2}
pub fn my_log(size: usize) -> usize {
    let mut copy_of_size: usize = size;
    let mut result: usize = 0;
    while copy_of_size != 1 {
        copy_of_size = copy_of_size >> 1;
        result += 1
    }
    // Make result even
    result = result + (result % 2);
    result 
}


//
//
//  BIT VECTOR METHODS
//
//
//
//
//
//
//
//
//

// Returns the size of the BitVector
pub fn bv_size(bit_vector: &BitVector) -> usize{
    bit_vector.size()
}



// Sets the value of the bit at index position with
// value 
pub fn bv_set(bit_vector:  &mut BitVector, index: usize, value: bool) {

    // make sure that the index is smaller than the size of the BitVector
    assert!(index < bit_vector.size(), "Index out of bounds");

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
        bit_vector.data[array_index] |= mask;
    } else {
        // if we want to change it to 0
        // we need to AND the zero at 
        // bit_index position and ones everywhere else
        // means we'll NOT the mask and AND
        // with the value at array_index
        bit_vector.data[array_index] &= !mask;
    }
}


// Gets the value at index
pub fn bv_get(bit_vector:  &mut BitVector, index: usize) -> bool { 
    // Check that the index is within the bounds
    assert!(index < bit_vector.size(), "Index out of bounds");

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
    (bit_vector.data[array_index] & mask) != 0
}






// Saves the bit vector to a file with filename
pub fn bv_save(bit_vector:  &mut BitVector, file_name: &str) -> std::io::Result<()> { 
    // Create the file
    let mut file = File::create(file_name)?;
    // Write the size
    file.write_all(&(bit_vector.size() as u64).to_le_bytes())?;
    // Write all the values from data
    for &value in &bit_vector.data {
        file.write_all(&value.to_le_bytes())?;
    }
    Ok(())
}


// Loads the bit vector from a file with file name
pub fn bv_load(file_name: &str) -> std::io::Result<BitVector> {


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
    Ok(BitVector { data, size })
    
}








//
//
//  RANK SUPPORT METHODS
//
//
//
//
//
//
//
//
//


// pub fn rs_rank1(bit_vector:  &mut BitVector,  superchunk_data: &mut Vec<u64>,  chunk_data: &mut Vec<u16>,  chunk_size: usize,
//     superchunk_size: usize, i: usize) -> u64 {
    
 
// }

