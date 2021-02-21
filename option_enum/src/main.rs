enum Option<T> { // <T> is generic type parameter: means the Some variant of Option can hold one piece of any data type
    Some(T),
    None,
}

fn main() {

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = Option::None; // if we use none type we have to specifiy variant since compiler cant infer type

    /* // cant do this since you cant add data type: i8 to enum<i8>; you have to convert Option<T> to T before performing T operations with it
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
    */


}
