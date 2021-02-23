fn main() {
    println!("Hello, world!");

    tard_function(69, 70, 'a', "TARD");//argument
    statements_expressions();

    let x = five(); // equivalent to let x = 5;
    let y = multiplied_by_3(32);
    println!("{} {}", x, y);

}

fn tard_function(x: i32, y:i32, z: char, tard: &str) { //parameter (MUST DECLARE EACH PARAMETERS TYPE)) { // THIS IS CALLED snake_case (lower case, and underscore for functions and variables)
    println!("Another function: {} {} {} {}", x, y, z, tard);
}

fn statements_expressions() { //RUST IS AN EXPRESSION LANGUAGE

    let r = 5; //a statement; does not return a value

    // let x = (let z = 3); returns error cuz statement doesnt return

    let t = { // an expression that evaluates to something like a math operation
        let k = 3;
        k + 1 //needs this even tho inside a scope // errors with semicolon since it makes it a statement
    }; //calling an expression, macro, block ( {} curly brackets)to create new scopes is an expression

    println!("{}", t);



}

fn five() -> i32  { // specifies return type ->
    //return 5
    5

    //can return early with return keyword
}

fn multiplied_by_3 (x: i32) -> i32 {
    x * 3 //cannot use semi colon or else its an expression; you can also do return statement if you want to
}