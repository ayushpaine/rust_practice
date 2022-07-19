use std::{
    clone,
    fmt::{Debug, Display},
};

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn Summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }

    fn Summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn Summarize(&self) -> String {
        format!("{}, by {}", self.username, self.content)
    }

    fn Summarize_author(&self) -> String {
        format!("{}", self.username)
    }
}

pub trait Summary {
    fn Summarize_author(&self) -> String;

    fn Summarize(&self) -> String {
        String::from("(Read more...)")
    } //default implementation, they can access other methods in trait definition
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("helmo"),
        content: String::from("hello there"),
        reply: false,
        retweet: false,
    }
}

/*pub fn notify(item: &impl Summary) {
    println!("breaking news! {}", item.Summarize());
}*/
// the above is basically syntax sugar for below which is a trait bound which means that the generic is limited to something
// that implements the summary trait
/*pub fn notify<T: Summary>(item: &T) {
    println!("breaking news! {}", item.Summarize());
}
*/
/*
pub fn notify(item1: &impl Summary, item2: &impl Summary) {}
*/

pub fn notify<T: Summary>(item1: &T, item2: &T) {
    //generic T implemenrs summary but both inputs should be of the same type
}

//for multiple types we use the where clause
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    10
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("largest number is x:{}", self.x);
        } else {
            println!("largest number is y:{}", self.y);
        }
    }
}

//blanket implementations- implementing traits on a type that impls another trait
impl<T: Display> ToString for T {}

pub fn run() {
    //traits allow us to share a set of methods that are shared across various types
    let tweet = Tweet {
        username: String::from("hemlo"),
        content: String::from("helu"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("laude"),
        headline: String::from("80 hajar"),
        content: String::from("ke shoes h"),
    };

    //notify(&article);

    println!("{}", returns_summarizable().Summarize())
}
