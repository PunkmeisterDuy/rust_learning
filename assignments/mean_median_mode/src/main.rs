use std::collections::HashMap;

fn main() {
    
    let nums : [i32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

    let mut vector : Vec<i32> = Vec::new();

    for element in nums.iter() {
        vector.push(*element);
    }

    let mut total : i32 = 0;

    for element in &nums {
        total = total + element;
    }

    let mean : f32 = (total as f32) / 2.0;


    let mut median;
    let length = vector.len();
    
    match is_even(&mut vector) {
        Some(true) => {
            let index = (length as f32) / 2.0;
            println!("{}", index)
            median = 0;

            println!("nums is even");
        },
        Some(false) => {
            let median = nums[length/2];
            println!("nums is odd");
        }
        ,
        None => {
            let median = 0;
            println!("Could not print median");
        }
    }

    


    println!("The mean is: {}", mean);
    println!("The median is: {}", median);


}

fn is_even(vector : &mut Vec<i32> ) -> Option<bool> {

    let length = vector.len();
    let modulus = length % 2;

    if (modulus == 0) {
        return Some(true);
    } else if (modulus == 1) {
        return Some(false)
    } else {
        return None
    }

}
