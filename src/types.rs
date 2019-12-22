/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/


// Rust is a statically typed language, which means that it
// must know the types of all variables at compile time.

pub fn run() {

    // default = i32;
    let x =1;

    // default f64;
    let y = 2.5;

    // add expl type 
    let z: i64 = 454545;

    // find max size 
    println!("Max i32: {}",  std::i32::MAX);
    println!("Max i64: {}",  std::i64::MAX);


    // boolian
    let foo:bool = true;

    println!("{:?}", (x, y, z, foo));

    // expression bool 
    let is_lower = 10 < 12;
    println!("is lower: {}", is_lower);

    // unicode char 
    let a1 = 'a';
    let smiley1 = '\u{1F600}';
    let smiley2 = '\u{1F916}';
    println!("{:?}", (a1, smiley1,smiley2,smiley2,smiley1));
}