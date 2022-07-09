/*
primitive strings: immutable fixed length string somewhere in memory
string: growable, heap-allocated data structure - use when u modify or own string data
*/

pub fn run() {
    let mut hello = String::from("hello");

    //get length

    println!("Length: {}", hello.len());

    hello.push(' '); //push char

    hello.push_str("world world"); //push string

    //capacity in bytes
    println!("Capacity: {}", hello.capacity());

    //check if empty
    println!("Is empty? {}", hello.is_empty());

    //check if contains a substring
    println!("Contains? {}", hello.contains("world"));

    //replace
    println!("Replace :{}", hello.replace("world", "there"));

    //loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    println!("{}", hello);

    //create string with capacity
    let mut s = String::with_capacity(10);
    s.push('A');
    s.push('B');

    //assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);
}
