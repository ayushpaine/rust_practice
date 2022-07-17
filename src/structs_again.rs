#[derive(Debug)]
struct User {
    username: String,
    email: String,
    count: u64,
    is_active: bool,
}

impl User {
    fn count_check(&self) -> u64 {
        self.count
    }
}

struct Color(i32, i32, i32); //tuple structs

pub fn run() {
    let mut user1 = User {
        email: String::from("ayushmt701@gmail.com"),
        username: String::from("ayush"),
        count: 1,
        is_active: true,
    };

    let name = user1.username;
    user1.username = String::from("80 hajar");

    let user2 = build_user(String::from("amogus"), String::from("soos"));

    let user3 = User {
        count: 2,
        is_active: false,
        ..user2
    };

    let color1 = Color(255, 255, 255);

    println!("{:#?}", user1);

    //to pass a struct as funciton parameter add a reference to the struct because we just want its fields and not ownership

    //to associate methods and functions we can create an impl block which can be multiple
    //methods -> have self as first arguement
    //associated functions -> dont

    println!("count :{}", user3.count_check());
}

fn build_user(email: String, username: String) -> User {
    User {
        email, // since field and values have same name we can use this shorthand
        username,
        count: 1,
        is_active: true,
    }
}
