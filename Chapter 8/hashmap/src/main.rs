use std::collections::HashMap; // least common of collections


fn main() {

    let mut scores = HashMap::new(); // hashes stored on heap ; also hashmaps can use values and keys from any type not just u32
    // however all values and keys must have the same type
    // basically this is like combining a vector and dictionary (from python)
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");
    scores.insert(blue, 10);
    scores.insert(yellow, 50); // ownership transfered ; with lifetimes: reference has to be alive as long as hashmap


    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let team_name = String::from("Blue");
    let score = scores.get(&team_name); // use get method, also this is an option cause of some and none

    match score {
        Some(scoring) => println!("{}", scoring),
        None => (),
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10); // only one key/value pair
    scores.insert(String::from("Blue"), 25);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50); // doesnt insert if it does exist

    println!("{:?}", scores);

    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect(); // we need <_, _> to know compile size and underscore _ for infering data type


        let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1; // we dereference count cuz it is &mut V
    } // we can use hashmap to look up its own value and then update itself

    println!("{:?}", map);

    // we can change default hash function as hasher implements BuildHasher trait

    }


