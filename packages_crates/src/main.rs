// THIS IS A BINARY FILE: exectuable // also a crate root

use rand;

//use std::cmp::Ordering;
//use std::io;
use std::{cmp::Ordering, self}; // combining them
//use std::io;
//use std::io::Write;
use std::{io, io::Write};

use std::collections::*; //brings all public items defined in a path into scope; usually used for tests

struct Rng {
    tard: u32,
}



fn main() {
    println!("Hello, world!");
    let idiot = Rng {
        tard: 5,
    };
}
