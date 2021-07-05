/*
Primitive Types--
Integer: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (Number of bits they take in memory)
(unsgined integers=>u (no negative values) , and normal integer=>i)(i32 is commanlly used)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

pub fn run(){
    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y  = 2.5;

    // Add explicit type
    let z: i64 = 399992444;

    // Find MAX size
    // println!("Max i32 is: {}", std::i32::MAX);
    // println!("Max i64 is: {}", std::i64::MAX);
    // println!("Max f64 is: {}", std::f64::MAX);

    // Boolean
    let is_active: bool = true;

    // :?=>debug tool
    println!("{:?}", (x,y,z,is_active));

    // Get Boolean from expression
    let is_greater: bool = 10==5;
    println!("{:?}", is_greater);

    // 
    let a1: char = 'a';
    let face =  '\u{1F600}';

    println!("{}",face);
}