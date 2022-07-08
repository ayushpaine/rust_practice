pub fn run() {
    //print
    println!("Hello from print!");

    //formatted strings
    println!("{} is from {}", "Ayush", "India");

    //positional arguements
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Ayush", "India", "Masti"
    );

    //named arguements
    println!(
        "{name} likes to do {activity}",
        name = "Ayush",
        activity = "Masti"
    );

    //placeholder traits
    println!("Binary: {:b}, Hex:{:x}, Octal:{:o}", 10, 10, 10);

    //placeholder for debug traits
    println!("This is {:?}", (12, false, "Helu"));

    //basic math
    println!("78*91 = {}", 78 * 91);
}
