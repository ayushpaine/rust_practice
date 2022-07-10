//reusable blocks of code, block scoped

pub fn run() {
    greeting("laude", "shoes");

    //bind fn output to variables;

    let sum = add(31, 56);
    println!("Laude {} hajar ke shoes h", sum);

    //closure
    let n3: i32 = 15;
    //can use outside functions in closures
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;

    println!("Laude {} hajar ke shoes h", add_nums(31, 49));
}

fn greeting(greet: &str, name: &str) {
    println!("{} 80 hajar ke {} h", greet, name);
}

fn add(n: i32, m: i32) -> i32 {
    n + m
}
