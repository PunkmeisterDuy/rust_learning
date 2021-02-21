/*
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 { //more efficent to group width and height together with tuple as width/heihgt are related to each other
    width * height
}



fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1 //confusing since we have to index our tuples
}
*/

#[derive(Debug)] // gives debug trait
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    //println!("{}", rect1); // wont work since there is no formatting "Display"
    //println!("rect1 is {:?}", rect1); //cannot use when there is no debug trait {:?} is debug;
    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1); // this looks better

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}



fn area(rectangle: &Rectangle) -> u32 { // defined with one parameter: immutable reference of the rectangle struct; also a reference as main still keeps ownership and so Rectangle is not transferred and scope is dropped in area function
    rectangle.width * rectangle.height
}