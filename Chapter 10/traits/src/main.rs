fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    /*
    let test = Testing {
        username: String::from("test"),
        content: String::from("tard"),
    };

    println!("{}", test.summarize());
    */

    println!("1 new tweet: {}", tweet.summarize());

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}


fn largest<T: PartialOrd + Copy>(list: &[T]) -> T { // adds Partial Ord and Copy trait to list
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub trait Summary {
    fn summarize_author(&self) -> String;
// I DONT KNOW HOW TO HAVE BOTH SUMMARIZE AND SUMMARIZE AUTHOR AT SAME TIME WITHOUT HAVING TO DECLARE FUNCTIONS FOR EACH IMPLEMENTATION
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
        // each type implementing this trait will be defined with this signature
        // however the impl is different
    }

}


pub fn notify(item: &impl Summary) { // uses a implemented struct with trait summary
    println!("Breaking news! {}", item.summarize());
}

/* 
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {

fn some_function<T, U>(t: &T, u: &U) -> i32 // for having many types 
    where T: Display + Clone,
          U: Clone + Debug
{
*/

//pub fn notify(item: &(impl Summary + Display)) { // implements two traits
// pub fn notify<T: Summary + Display>(item: &T) { other syntax

// pub fn notify<T: Summary>(item1: &T, item2: &T) {
// could share same type with this syntax

/*
pub fn notify<T: Summary>(item: &T) { // more verbose version
    println!("Breaking news! {}", item.summarize());
}
*/

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

/*
impl Summary for NewsArticle { // Summary for NewsArticle implements trait
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location) // this overides trait
    }
}
*/

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

struct Testing {
    username: String,
    content: String,
}

/*
impl Summary for Testing {
}
*/

fn returns_summarizable() -> impl Summary { // returns a trait as Tweet has Summary trait
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

/*
fn returns_summarizable(switch: bool) -> impl Summary { doesnt work cause you cant return multiple traits u can do in chapter 17 though
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}
*/


use std::fmt::Display;

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
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

/*
impl<T: Display> ToString for T {
    could do this since Display implements ToString on any type that implements Display trait
}

let s = 3.to_string();


*/