// THIS IS A LIB(RARY) FILE: dependency// a crate root


mod front_of_house { // a parent module WOWW // can also hold defintions for other stuff like structs, enums, constants, traits, and functions
    pub mod hosting { // easy to navigate // child of front of house
        pub fn add_to_waitlist() {} // a sibling // needs to be pub also since encapulation

        fn seat_at_table() {} // also a susbling 
    }

    mod serving {

        mod dumb {
             // also encapulation can happen: take_order can use servings stuff and hosting (ancestors properties, but parents cant use this stuff)
        }

        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
        
    }

    fn test() {
        //eat_at_resturant(); doesnt work cuz super is like ../
        super::eat_at_resturant()
    }


}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String, // automatically private 
    }

    pub enum Appetizer {
        Soup, // automatically public
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

mod example;

pub use crate::hosting::add_to_waitlist;

pub use crate::front_of_house::hosting; // also can use public to rexport (bring itme to scope but also making public for others)



pub fn eat_at_resturant() {
    crate::front_of_house::hosting::add_to_waitlist(); // absolute

    front_of_house::hosting::add_to_waitlist; // relaitve

    hosting::add_to_waitlist(); // we can use since we used the crate:: front of housing

    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

fn main() {
use std::fmt::Result;
use std::io::Result as IoResult; // renames as IoResult and doesnt conflict

fn function1() -> Result {
    // --snip--
    Ok(())
}

fn function2() -> IoResult<()> {
    // --snip--
    Ok(())
}
}