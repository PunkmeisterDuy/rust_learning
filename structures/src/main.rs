fn main() {
    println!("Hello, world!");

    let mut user1 = User { // can only make entire structure instance mutable, not only certain parts
        email: String::from("email@protonmail.com"),
        username: String::from("Punkme"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("this@email.com");
    println!("{}", user1.email);

    /* inneffective doing user1.active/ user1.sign_in_count
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    */

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };


    // tuple struct
    let black = Color(0, 0, 0);

    black.2;

    

}

// regular struct with name of struct, and data fields with names
struct User {
    username: String, // If &str, compile error: expected lifetime parameter.uses strings because it wants to have string in its entire lifetime and not reference to be owned by sdomething else
    email: String,
    sign_in_count: u64,
    active: bool
}

//tuple struct: give name, but data fields without names; still defined by its own strut type so cannot be used exactly as tuple
struct Color(i32, i32, i32);

/*
fn build_user(username: String, email: String) -> User { // innefficent to fill in each time (username: username, email: email etc...)
    User {
        username: username,
        email: email,
        active: true,
        sign_in_count: 1,
    }
}
*/

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}