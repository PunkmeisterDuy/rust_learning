fn main() {

    let mut x =3;
    if x < 3 {
        println!("Tard"); //sometimes called arms
    } else {
        println!("false")
    }

    /*
    let y = 2;
    if y { // expects boolean (will not automaticallly convert non booleans to a boolean)
        println!("works");
    } else {
        println!("no it doesnt duimb");
    }
    */

    let mut z = 5;
    if z == 5 {
        println!("Tard5") // goes to first condition
    } else if z !=0 {
        println!("Tard") // use match  for big if elses
    }

    let r = if true { 5 } else { 6 }; //evaluates as an expression on statement
    println!("{}", r);
    // let t = if true { 5 } else { "five" }; error since results from each arm needs to be same type
    // compiler needs to know at compile time what data type is t

    loop {
        println!("cause its endless"); //can use loop to retry operation that you might thiink will fail
        break // like checking if a thread completes a job or apache stops working
    }

    let result = loop {
        z += 1;
        if z > 10 { //same as while loop
            break z
        }
        println!("{}", z)
    };

    while x > 0 {
        x -=1;
        println!("{}", x);
    }

    //looping through a collection wtih for
    let array = [10, 20, 30, 40, 50];
    for element in array.iter() {
        println!("{}", element); // could use a while loop but prone to erros and performance shit
    }


    for number in (1..4).rev() { //idk the syntax of this dumb
        
        println!("{}", number);

    }

}
