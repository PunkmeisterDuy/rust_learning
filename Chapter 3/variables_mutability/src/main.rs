fn main() {

    let x = 5;
    //x = 4 will not work
    
    let mut y = 6;
    y = 7;
    println!("{}", y);

    const MAX_SCORE : u32 = 100 * 100;
    //const MAX_SCORE : u32 = 100 * y; will not work since assigned to non constant expression 
    println!("{}", MAX_SCORE);

    let z = 3; //you can also use mut before shadowing
    let z = 4; //shadowing uses let

    /*
    let mut spaces = "   ";
    spaces = spaces.len(); // not allowed since you are not allowed to mutate a variable's type
    */ 
    println!("{}", z);

}
