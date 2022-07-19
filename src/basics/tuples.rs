/*
tuples group together elements of different types
max 12 elements
to get data - use destructuring or . notation
*/

pub fn run() {
    let person: (&str, &str, i8) = ("Ayush", "India", 19);

    println!("{} is from {} and is {}", person.0, person.1, person.2);
}
