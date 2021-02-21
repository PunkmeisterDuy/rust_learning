fn main() {


    let s = String::from("hello world");
    let hello = &s[0..5]; // this is a string slice which references portion of string [starting postion, ending position + 1]; also the " .. " is range syntax operation
    println!("{}", hello);

    //&String is a reference to a string &str is a string slice (reference portion)

    let a = [1, 2, 3];
    let array_slice = &a[0..2];
    for element in array_slice.iter() {
        println!("{}", element)
    }


}

fn first_word(s: &str) -> &str { //could do first_word(s: &String) but it limits it to Strings so we should do &str which can do both strings and string slices

    let bytes = s.as_bytes(); //converts into an array of bytes

    for (i, &item) in bytes.iter().enumerate() { // iterates the array using iter() (returns each element in a collection that enums result of each iteration and returns each element as tuple (enum is list of special values))
        //enum returns index and then reference to element (i, &item) for example
        if item == b' ' { // b' ' stands for byte literal of space
            return &s[0..i];
        }
    }

    &s

}
