//iterate on a condition

pub fn run() {
    let mut _count = 0;

    for x in 0..100 {
        if x % 10 == 0 {
            println!("80: {}", x);
        } else if x % 15 == 0 {
            println!("Hajar: {}", x);
        }
    }
}
