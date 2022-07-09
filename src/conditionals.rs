//conditionsla - used to check a condition and act on the result

pub fn run() {
    let age: u8 = 18;
    let check: bool = false;
    let knows: bool = true;

    //if/else
    if age >= 21 && check || knows {
        println!("Laude {} hajar ke {} h", 80, "shoes");
    } else if age < 21 && check {
        println!("Tera {} jainga isme", "ghar");
    } else {
        println!("Tera {} {} jainga", "ghar", "chale");
    }

    //shorthand

    let is_of_age = if age >= 21 { true } else { false };

    println!("Is of age: {}", is_of_age);
}
