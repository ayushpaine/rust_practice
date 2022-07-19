//variables hold primitive data or references to data
//are immutable by default
//block-scoped:rust
//can redeclare vars while preserving mutability - shadowing

pub fn run() {
    let name = "Ayush";
    let mut age = 19;

    println!("My name is {} and I am {} years old", name, age);

    age = 18;

    println!("My name is {} and I am {} years old", name, age);

    const ID: i32 = 001; //explicitly state type for constants
    println!("ID: {}", ID);

    //assign multiple values
    let (my_name, my_age) = ("Ayush", 19);
    println!("{} is {}", my_name, my_age);
}
