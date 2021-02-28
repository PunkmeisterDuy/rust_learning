#[cfg(test)] // i ran cargo new adder --lib
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    /*
    #[test]
    fn another() {
        panic!("Make this test fail");
    }
    */

    // assert! macro checks if something equals true
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller)); // returns true
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
        //checks if left == right
        // called expected and actual in other languages
    }

    /* this is an example of fail so keep it
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        /* this is an example of fail so keep it
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
        */
    }
    */

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")] // if it doesnt panic something is wrong
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four")) // use err instead of should panic
        }
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}
/*

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name);
}
*/

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

/*
assert_eq! checks if equal
assert_ne! checks if inequal
*/