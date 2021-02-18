fn main() {

    let guess: u32 = "42".parse().expect("Not a number!"); // if without a data type, it will give error because compiler cant infer what type


    //THESE ARE  SCALAR VALUES MEANING ONE VALUE
    let x : i16 = -3;
    let y : u32 = 3; //has to be positive
    let tard : isize = 5999990; //goes to 64 bit because based off cpu archietcture

    let ingram : f32 = 2.0; // or f64
    let t = true;
    let f: bool = false; // with explicit type annotation

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    //COMPOUND TYPES GROUP MULTIPLE VALUES TO ONE TYPE

    let tup: (i32, f64, u8) = (500, 6.4, 1); // fixed length, cannot grow or shrink; dont need to have to declare different type for each

    let tupez = (500, 6.4, 1);

    let (r, t, k) = tupez; //CALLED Destructuring because it breaks tuple into many parts

    println!("The value of y is: {}", t);
    println!("{}", tupez.0);

    let array = [1,2,3,4,5]; // fixed length (use vector if want to grow/shrink), but all have to be same type; and allocated on stack(faster) than heap

    let a: [i32; 5] = [1, 2, 3, 4, 5]; //i32 type fixed length 5
    let b = [3; 5]; //creates same value for each element: 3 with fixed length 5
    //let element = b[20]; // will create a runtime error (compile error is violating syntax; run time caused when running); usually other low-level languages access random memory


}
