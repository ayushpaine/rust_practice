/*
fixed list with elements of same data type
*/

use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    //re-assign values
    numbers[2] = 10;

    println!("{:?}", numbers);

    //single value
    println!("{}", numbers[0]);

    //get length
    println!("Length: {}", numbers.len());

    //arrays are stack allocated
    println!("This array occupies {} bytes", mem::size_of_val(&numbers));

    //get slices
    let slice: &[i32] = &numbers[1..2];
    println!("Slice: {:?}", slice);
}
