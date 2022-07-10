//create custom data types

//tuple struct

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    //construct Person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    //get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    //set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    //name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut p = Person::new("Ayush", "Paine");
    println!("Person {}", p.full_name());
    p.set_last_name("80 Hajar");
    println!("Person {}", p.full_name());
    println!("Person {:?}", p.to_tuple());
}
