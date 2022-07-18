use std::{collections::HashMap, hash};

pub fn run() {
    let blue = String::from("blue");
    let red = String::from("red");

    let mut scores = HashMap::new();

    scores.insert(blue, 10); //this will move the ownership of vars blue and string into the hashmap
    scores.insert(red, 20);

    println!("{:#?}", scores);

    let blue_score = scores.get("blue");
    println!("{:#?}", blue_score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.insert("blue".to_string(), 25); //overwrites existing key

    scores.entry("red".to_string()).or_insert(30); //dont override if there is an already existing value

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", map);
}
