

    // Option<T>; the <T> is a generic


    /*
fn largest(list: &[i32]) -> &i32 { // we removed duplicating code with a function
    let mut largest = &list[0]; // we can do better with generics for adding functionality with other types like chars

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
}

*/

//ALSO NO RUNTIME PERFORMANCE LOST WHEN USING GENERICS

struct Point<T, U> { // <T, U> allows them for being different types
    x: T,
    y: U,
}

struct Soint<T> {
    x: T,
    y: T,
}

impl<T> Soint<T> {
    fn x(&self) -> &T {
        &self.x // declaring &self.x immediately so rust can identify type is generic instead of concr4ete
    }
}
impl Soint<f32> { // only on instances of F32 instead of T
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


enum Result<T, E> {
    Ok(T),
    Err(E), // holds the values of the things
}

fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0]; // doesnt work cause some types cant be ordered so give PaartialOrd trat error

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}



fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    let s = Soint { x: 5, y: 10 };

    println!("p.x = {}", s.x());
}