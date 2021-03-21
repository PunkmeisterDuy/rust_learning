use std::io;

fn main() {
    println!("Celsius or Fahrenheit (Celsius/Fahrenheit)");

    let mut temp_type = String::new();

    io::stdin()
    .read_line(&mut temp_type)
    .expect("failed to read");

    let temp_type : &str= &temp_type.trim().to_lowercase();

    if !(temp_type.eq("celsius") | temp_type.eq("fahrenheit")) {
        println!("please type correctly dumb");
    }


    let mut temp = String::new();
    println!("Enter temp: ");

    io::stdin()
    .read_line(&mut temp)
    .expect("failed to read num");

    let temp : i32 = temp.trim().parse().expect("dumb");
    

    if temp_type.eq("celsius") {

        let temp = (temp * 9 / 5) +32;
    } else if temp_type.eq("fahrenheit") {

        let temp = (temp - 32) * (5/9);
    } else {

        print!("u dumb");
    }

    println!("Your {} is {}", temp_type, temp )

}
