use rand::{CryptoRng, ErrorKind::Transient, Rng}; //nested paths
use std::io::{self, Write}; //self for io(parent)
                            //use * for all public items in a parent to bring into scope

fn serve_order() {}

mod front_of_house;

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); //super allows us to reference the parent module
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub use self::front_of_house::hosting; //relative path
                                       //use crate::front_of_house::hosting; absolute path
                                       //add pub keyword infront of use to allow external files access to it
pub fn eat_at_restaurant() {
    //absolute path
    // crate::front_of_house::hosting::add_to_waitlist(); child modules by default private add pub keyword even functions inside a pub module
    crate::front_of_house::hosting::add_to_waitlist();

    //relative path
    // front_of_house::hosting::add_to_waitlist();

    // let mut meal = back_of_house::Breakfast::summer("peanuts"); breakfast and summer pvt by default(for structs and enums)
    let mut meal = back_of_house::Breakfast::summer("peanuts");
    //meal.toast = String::from("peepee"); fields in a struct also pvt even if the struct is pub however values of an enum are public
    meal.toast = String::from("peepee");

    let order1 = back_of_house::Appetizer::Salad;
    let order1 = back_of_house::Appetizer::Soup;

    front_of_house::hosting::add_to_waitlist(); //use keyword allows to bring a path into scope
    hosting::add_to_waitlist() //now can directly use hosting
}

use std::fmt::Result;
use std::io::Result as IOResult; //to bring same Result names but diff types rename them or you can just import their parents
