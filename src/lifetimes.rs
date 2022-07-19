use std::result;

struct ImportantExcerpt<'a> {
    part: &'a str, //our struct cant outlive the reference passed into the struct
}

impl<'a> ImportantExcerpt<'a> {
    fn return_part(&self, announcement: &str) -> &str {
        println!("attention: {}", announcement);
        self.part
        //no need to specify lifetime annotations, see elision rules
    }
}

pub fn run() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    // println!("{}", r); //r is a dangling ref as x is inner scoped and after scope ends it gets invalidated
    //the borrow checker does this

    let string1 = String::from("hello1");
    let string2 = String::from("hello2");

    let result = longest(string1.as_str(), string2.as_str());
    println!("longest :{}", result);

    let novel = String::from("omk. kk");
    let first_sentence = novel.split('.').next().expect("emrror");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    let s: &'static str = "static";
    //static lifetime means ref can live long as duration of program, all string literals have a static lifetime
    //as they are stored in the program binary
}

//&i32 -> a reference
//&'a i32 -> a reference with an explicit lifetime
//&'a mut i32 -> a mutable reference with an explicit lifetime

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    //x and y could have diff lifetimes so this cant be detected by the borrow checker
    //so lifetimes should be specified, use generic lifetime annotations(describe lifeitmes of multiple refs and how they relate to each other)
    // general syntax is "'{alphabet}"
    if x.len() > y.len() {
        x
    } else {
        y
    }
    //this means that the lifetime of the return value will be equal to the smallest value of the inputs'
}

fn longest2<'a>(x: &str, y: &str) -> &'a str {
    let ans = String::from("ok");
    ans.as_str() //cant do this as it references local function variable as when fn gets over local variable gets dropped
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if (item == b' ') {
            return &s[0..i];
        }
    }
    &s[..]
}
//here we dont get any errors and out code compiles correctly as in some situations because of 3 lifetime elision rules:
//1. each parameter that is a reference gets its own lifetime parameter
//2. if there is exactly one input lifetime parameter, that lifetime is assigned to all output parameters
//3. if there are multiple input lifetime parameters, but one of them is &self or &mut self then the lifetime of self
//is assigned to all output lifetime parameters
