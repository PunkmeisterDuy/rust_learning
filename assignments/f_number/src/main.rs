use std::io;

fn main() {


    let mut f_array = vec![0, 1];
    
    let mut input = String::new();

    println!("enter f_number index");

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    

    let input : usize= input.trim().parse().expect("no number");
    

    for number in (2..input +1) {
        println!("{}", number);
        f_array.push(f_array[number - 1] + f_array[number - 2]);
    }
    /*
    if input > 2 {
        let input = input - 2;

        for number in (2..input) {
            println!("{}", number);
            f_array.push(f_array[number - 1] + f_array[number - 2]);
        }
    }
    */

    println!("Hello, world!");

    for element in &f_array {
        println!("{}", element);
    }

    println!("your f_number is: {}", f_array[input])

}
