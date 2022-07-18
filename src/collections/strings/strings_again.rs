use unicode_segmentation::UnicodeSegmentation;

pub fn run() {
    //strings are stored as a collection of utf 8 encoded bytes
    let s1 = String::from("hello");
    let s2 = "hello";
    let s3 = s2.to_string();
    let mut s4 = String::new();

    s4.push_str("hello");
    s4.push('p');

    let s5 = s1 + &s3; //ownership of s1 moved to s5
    let s6 = format!("{}{}", s2, s5); //doesnt take ownership of these strings

    let hello = String::from("नमस्ते");
    //string cant be indexed by numbers
    //bytes, scalar values, grapheme clusters can be used for indexing
    //for grapheme cluster add unicode-segmentation crate
    for b in hello.bytes() {
        println!("{}", b);
    }

    for c in hello.chars() {
        println!("{}", c);
    }

    for g in hello.graphemes(true) {
        println!("{}", g);
    }
}
