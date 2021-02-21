fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); //ampersand is referencing we trake &String not String type; we can also you dereference operator " * ";

    //let &s1 = String::from("tarded"); wont work cause we cant modify a refference; code doesnt work but imagine if &s1 is mut it would work; reference immutable default

    //let mut tard = &mut s1;
    //let mut tung = &mut s1; unable to do since you can only have one mut reference in a scope; prevents data RACESSSS!!!!; rust wont compile data races
    // could create different scope for multiple mut ref

    //let dumb = &s1;
    //let did = &s1;
    //let idiot = mut &s1; // errror cause we cant have both immutable and mutable references; can be used if dumb are dropped from mem

    //also compiler dis allows dangling references when reference is allocated to a variable whos ownership already transfered (something about lifetimes)

    println!("The length of '{}' is {}.", s1, len); // much better than transfering ownership to function because thats dumb 
}

fn calculate_length(s: &String) -> usize {
    s.len()
}