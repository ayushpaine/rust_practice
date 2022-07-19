/*
ownership rules ->
each value in rust has a variable thats called its owner
there can be only one owner at a time
when the owner goes out of scope the value will be dropped
*/

pub fn run() {
    {
        //s is not valid here as undeclared
        let _s1 = "hello"; //s is valid from this point on
                           //do stuff with s
                           // this scope is over now and s is no longer valid
                           //string literals directly stored in binary and fixed in size
        let _s2 = String::from("hello"); //heap allocated memory
    }

    let x = 5;
    let _y = x; //copy trait for types like int bool char allows to be copied

    {
        let s1 = String::from("hello");
        let _s2 = s1; //move not shallow copy as it invalidates x while doing so

        // println!("{}", s1);
    }

    let s1 = String::from("hello");
    let _s2 = s1.clone(); //clone method
    println!("{}", s1);

    let ss = String::from("hello");
    takes(ss);
    //println!("{}", ss); //function takes ownership and at its end it drops the variable(same as assinging ss to another variable)

    let ss1 = gives();
    println!("{}", ss1);

    let ss2 = String::from("hello");

    let ss3 = takes_and_gives(ss2);

    println!("ss1: {}, ss2: {}", ss1, ss3);

    //to use a var and not take owneership we use refs

    let sss1 = String::from("hello");
    let len = calculate_length(&sss1);
    println!("length {}", len);

    let mut ssss1 = String::from("hello"); //mutable refs can be used to change the value without taking ownership
    change(&mut ssss1); //IMP:can only have one mutable reference to a mutable data in a particular scope
                        //this helps in preventing data races at compile time, 2 pts are pointing to the same data and
                        //one of them is sued to write data and no mech to synch data so data corrupted
                        //also u cant have a mutable reference if an immutable ref already exists but mutliple immut allowed
    println!("{}", ssss1);

    //dangling refs: ref to invalid data
    //if we return a ref to a variable outisde its scope which means it gets dropped from heap so it will point to invalid mem

    //REFERENCES MUST BE VALID ALWAYS

    let mut sssss1 = String::from("hello world");
    let hello = &sssss1[..5];
    let world = &sssss1[6..];

    //let word = first_word(&sssss1); //string ref gets auto coerced to str slice
    // sssss1.clear(); //mutates string cant mix mut and immut in the same scope

    let sssss2 = "hello world"; //this is a string slice, its a string literal

    let word = first_word(sssss2);

    //slices can be on diff types of collections
    let a = [1, 2, 3, 4, 5];
    let slice = &a[0..2];
}

fn takes(string: String) {
    println!("{}", string)
}

fn gives() -> String {
    let some_string = String::from("hello");

    some_string
}

fn takes_and_gives(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    let length = s.len(); //passing ref without taking ownership is called borrowning , refs are immputable by default

    length
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
