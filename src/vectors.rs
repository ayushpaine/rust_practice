/*
resizable arrays
*/

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    //re-assign values
    numbers[2] = 10;

    //add to vector
    numbers.push(16);

    //pop last value
    numbers.pop();

    println!("{:?}", numbers);

    //single value
    println!("{}", numbers[0]);

    //get length
    println!("Length: {}", numbers.len());

    //arrays are stack allocated
    println!("This vector occupies {} bytes", mem::size_of_val(&numbers));

    //get slices
    let slice: &[i32] = &numbers[1..2];
    println!("Slice: {:?}", slice);

    //loop through vector values
    for it in numbers.iter() {
        println!("Number: {}", it);
    }

    //loop and mutate values
    for it in numbers.iter_mut() {
        *it *= 2;
    }

    println!("Numbers vector: {:?}", numbers);
}
