fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { // 'a is a generic
    if x.len() > y.len() {
        x // 'a means that x stays in lifetime till it is returned
    } else {
        y
    }
}


/*
'a represents a longer block than block 'b

&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
*/