fn main() {
    let mut s = String::new(); // or do this samthing

    let data = "initial contents";

    let s = data.to_string(); //  this adds the display trait like string literals

    // the method also works on a literal directly:
    let s = "initial contents".to_string(); // can either do this

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";


    s1.push_str(s2); // takes a string slice cuz we dont want to take ownership
    println!("s1 is {}", s1);
    println!("s2 is {}", s1);


    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    //also learn more about dereferencing coercion cuz it can convert a &String to a &str

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3); // format doesnt take ownership and reutrns a string

    //let s1 = String::from("hello"); // a string is a wrapper  of Vec<u8>
    //let h = s1[0]; //cant do this since we cant index with integer
    
    //let hello = "Здравствуйте";
    //let answer = &hello[0]; //rust doesnt let you compile cuz utf 8 has different byte stuff, uses multiple bytes per char


    let hello = "Здравствуйте";

    let s = &hello[0..4]; // overall iterating strings with index and slices are both dangerous

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }// could do this for iterating
}
