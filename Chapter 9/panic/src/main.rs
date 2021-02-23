use std::fs::File;
use std::io;
use std::io::Read;
use std::io::ErrorKind;
use std::fs;
use std::net::IpAddr;



// btw this code is unrunnable because of too many panics and i dont want to fix them
// panic stops excetuion of program

fn main() {
    let f = File::open("hello.txt"); //btw unwrap is short cut for matching with Result.
    let u = File::open("hello.txt").unwrap(); //calls panic! macro automatically and ok will unwrap the value
    // could also use expect as you can provide goode error messages
    let x = File::open("hello.txt").expect("Failed to open hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };


    let home: IpAddr = "127.0.0.1".parse().unwrap(); // this is fine since it is static and wont chagne

}

fn read_username_from_file() -> Result<String, io::Error> { // propagates error somewhere else
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_easy() -> Result<String, io::Error> { // this is the much shorter way for propagating
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?; // use the ? for propagation // if Ok returns Ok(s), elif Err(e)
    // this is useful if you have one error type to represent all the ways a function might fial
    // ? can only be used with things that return Result or Option enum//
    Ok(s)
}

fn read_username_from_file_even_easier() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt") // in built function
}

/* could also you if statements
fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
*/


pub struct Guess {
    value: i32,
}

impl Guess { // for the guessing game validating an i32
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value // used as getter since Guess struct is private
        // value is private so Guess doens't access and modify itself
    }
}