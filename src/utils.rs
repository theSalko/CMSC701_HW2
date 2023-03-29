

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