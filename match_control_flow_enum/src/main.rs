enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin { // with an if statement,the expression needs to return a boolean; match gives the returned value
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }, //returns 1; could return any type
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // have to use none since compiler doesnt know what to do with nOne
        Some(i) => {
            println!("{}", i+1);
            Some(i + 1)
        },
    }
}

fn main() {

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?} {:?} {:?}", five, six, none);

    let some_u8_value = 1;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    println!("{}", value_in_cents(Coin::Quarter(UsState::Alaska)));
    
}
