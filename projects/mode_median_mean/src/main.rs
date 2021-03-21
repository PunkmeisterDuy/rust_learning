// TODO fix the fixed list length ; probs with <T> or something
// TODO even numbered lists
// TODO lists with multiple modes
// TODO modulate program
// TODO fix conversion
// TODO unsafe Option changing

use std::collections::HashMap;

fn main() {
    let list: [i32; 5 ] = [0, 1, 3, 3, 4];

    let mut vec = convert_list_vec(list);

    let mean = mean(&vec);
    let median = median(&vec);
    let mode = mode(&list); 

    println!("Mean: {}", mean);
    println!("Median: {}", median);
    println!("Mode: {}", mode);
    println!("Hello, world!");
}

// TODO: convert this list without saying list length
fn convert_list_vec(list : [i32; 5]) -> Vec<f32> {

    let mut vec: Vec<f32> = Vec::new();

    for i in list.iter() {
        vec.push(*i as f32);
    }
    vec
}

fn mean(vec : &Vec<f32>) -> f32 {
    let mut mean : f32 = 0.0;


    for i in vec {
        mean = mean +i;
    }
    mean / (vec.len() as f32)
}

// TODO: Try converting f32 type with T
// TODO: Also try using Option or Some i forgot the difference between the enums
fn median(vec : &Vec<f32>) -> f32 {

    let mut median : f32 = 0.0;
    let length : usize = vec.len();
    // could do a match but im not gonna do that
    if (length % 2) > 0 {
        median = vec[(length/2)]
    }
    else if (length % 2) ==0 {
        return 0.0 // i forgot how to do this
    }
    median
}

fn mode(list : &[i32; 5]) -> i32 {

    let mut hash = HashMap::new();
    for i in list.iter() {
        let count = hash.entry(i).or_insert(0);
        *count +=1; //ick how this works 
    }
  // https://stackoverflow.com/questions/30414424/how-can-i-update-a-value-in-a-mutable-hashmap ;
  // maybe this will explain idk how it works still after reading it 
    let mut mode : i32 = 0;
    for i in list.iter() { // also learn option and if let cuz idk how unwrap works
        let value : i32 = *hash.get(i).unwrap();
        println!("{}, {}",i, value); 
        if value > mode {
            mode = *i; 
        }
    }

    println!("{:?}", hash);
    // for i in vec insert the vector values, but how does a hashmap count each time a value is
    // placed there; it needs a key value pair or a key with no value; could each i be a key and
    // with a value of 1 and then each time that key adds to it: it simulates as a counter and then
    // pick the key with the greatest value and return it; so how do you code that

    mode

}
