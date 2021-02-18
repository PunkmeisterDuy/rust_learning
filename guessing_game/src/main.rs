use std::io;
use rand::Rng; //a trait imported
use std::cmp::Ordering; //also a trait

fn main() {
    
    let _secret_number = rand::thread_rng()
        .gen_range(1, 101); //uses lazy thread rng method and generated between 0 - 100

    println!("The secret number is {}", _secret_number); //remove or leave this here i dont care

    loop {

        println!("Input guess");

        let mut guess = String::new(); //like a static method, actually is one, doesn't use self. something
        
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read"); //io::Result gives an enum Ok or Err and needs .expect method or wonk

        let guess: u32 = match guess.trim().parse() { //a match expression
            Ok(num) => num, //ARMS
            Err(_) => {
                println!("Enter a number please");
                continue
            },
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&_secret_number) { //an expression that uses arms of patterns
            Ordering::Less => println!("Too small!"), //uses a pattern
            Ordering::Greater => println!("Too big"), //this is also a pattern
            Ordering::Equal => {
                println!("I WIN WHAT!");
                break;
            }, //also pattern
        }
    }


}
