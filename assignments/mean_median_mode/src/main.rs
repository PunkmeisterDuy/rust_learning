fn main() {
    
    let nums : [i32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

    let mut vector : Vec<i32> = Vec::new();

    for element in nums.iter() {
        vector.push(*element);
    }

    let mut total : i32 = 0;
    let mut num_counter  = 0;

    for element in &nums {
        total = total + element;
    }

    let median : i32;
    

    
    match is_even(vector) {
        Some(true) => {
            println!("nums is even");
        },
        Some(false) => {
            let median = nums[nums.len()/2];
            println!("nums is odd");
            println!("{}", median)

        }
        ,
        None => println!("Could not print median")
    }
    
    let mean : f32 = (total as f32) / 2.0;


    println!("The mean is: {}", mean);


}

fn is_even(vector : Vec<i32> ) -> Option<bool> {

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
