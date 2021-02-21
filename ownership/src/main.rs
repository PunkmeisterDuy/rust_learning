fn main() { //sample is not valid since not declared

    //garbage collection constantly looks for no longer used memory
    // c and c++ must explicitly allocate/free memory or mem leak
    let sample = "hello"; // "hello" is a string literal; not mutable

    let mut s = String::from("hello"); // creates String from a string literal stored on heap

    // String allocates on heap since it is mutable as the space can increase/decrease so compiler doesnt know size; allocated at run time
    // we need to explicitly return it since no Garbage Collector cuz we need to allocate and free at right times

    s.push_str(", world!");


    println!("{}", sample);

    let mut number : i32 = 5; // this is popped off stack (chapter 3 data types) after scope is over
    let same_number = number; // value is copied since its on the stack and can afford to have duplicate data; we dont need to worry about deep/shallow/cloning 
    let number = 3; //we can apply annotation copy trait on types like integers on the stack (idk but this is a cliffnote whatever that is) ALL INTEGERS, BOOLEANS, FLOATING POINTS, CHARs, TUPLES can implement copy (tuples can only if their types have copy also)

    println!("{}", same_number);


    let string_copy = String::from("no");
    let not_copy = string_copy; // we copy the ptr, len, capacity, but not actual heap contents cuz expensive if you copy huge amounts of data; this (shallow copies:java terminology) move(s):rust terminalogy, but also invalidates string_copy 
    // RUST NEVER DOES DEEP COPIES WITHOUT PERMISSON

    println!("{}", not_copy);
    //println!("{}", string_copy); will not work since rust moves the memory to not_copy

    //we can clone tho to deep copy
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2= {}", s1, s2);

    //let example_ownership = String::from("tard");
    //takes_ownership(example_ownership); // example_ownership is no longer valid
    // take ownership drops the memory
    // you could copy if you do copy = take_ownership(example_ownership); and take_ownership returns example _ownership to transfer ownership
    // to avoid being retarded by having to copy and than return values back we use references

    // let num = 3;
    //example_fun(num); // this works since num has Copy trait
    // functions parameter is dropped but not actaully num





} // scope of sample over, and s is invalid; memory of s is dropped as variable goes out of scope
