fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);


    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    let s: &'static str = "I have a static lifetime."; // says it lasts entire program, but its for compiler
    // you have to check if the lifetime doesnt die and create dnagling references
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { // 'a is a generic
    if x.len() > y.len() {
        x // 'a means that x stays in lifetime till it is returned
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str, // we have to add a lifetime for each reference
}

fn first_word(s: &str) -> &str { // wouldnt run in past since every reference needed explicit lifetimes
    // compiler predicts using elision rules
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str { // works cuz of elision rule
        println!("Attention please: {}", announcement);
        self.part
    }
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>( //'a IS A GENERIC WOWWWW
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where // this code uses generics, traits, and lifetimes
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/*
'a represents a longer block than block 'b

&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
.

fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
} this is not valid since lifetime is not related in this function

lifetimes on function/method parameters are called input lifetimes

lifetimes on return values are output liftimes
compiler has 3 rules for knowing if lifetime is 
reread chapter 10.3 cuz my head is dead right now and im going to rewrite notes after i read entire book
*/

