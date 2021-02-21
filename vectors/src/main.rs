fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(6);
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    for i in &v { // iterates over each mutable references
        println!("{}", i); // we cant use a += thing like a counter before dereferencing
    }
    
    // vector goes out of scope when freed (all mem including elements); i think vecs are structs idk tho
    let mut easy = vec![1,2,3]; // a macro that infers type

    //let first = &easy[2]; // cant do this since first.push need immutable reference idk why its not failing
    easy.push(2);
    //println!("The first element is: {}", first); // this also with first

    let third: &i32 = &v[2]; //reference so it doesnt do dumb so you dont transfer ownership
    println!("The third element is {}", third);

    match v.get(100) {
        Some(third) => println!("The third element is {}", third), // forgot how this works but go to 6.2 for Option type enums
        None => println!("There is no third element."), // better this way since you have some and none so they dont break your dumb indexing
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![ // all the same type: SpreadsheetCell
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

}
