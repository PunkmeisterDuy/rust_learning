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


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle { // also this is a method not a function since its in context of a struct (or enum, or trait object) (first parameter is always self)
    fn area(&self) -> u32 { // use reference since we dont want area to take ownership
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle { // you dont need to split this up, but this shows that you just can: useful for generic types and traits
    fn square(size: u32) -> Rectangle { //not a method since it doesnt use self so it is a function (we use :: syntaxto call associated function)
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let rect_square = Rectangle::square(32);
    println!("{:#?}", rect_square);

    // p1.distance(&p2); cleaner than
    // (&p1).distance(&p2); this since rust can infer much better with method recievers as it can read if &self is mutating or consuming(meaning u use and drop memory)

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}