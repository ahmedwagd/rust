// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Ahmed"; // like const
    let mut age = 28;   // like let can redeclared

    println!("My name is {}, and my age is {}",name,age);

    age = 29;
    println!("My name is {}, and my age is {}",name,age);

    // Define constant
    const ID: i32 = 001; // this is intger number with 32 bits Define as a constant usually use upper_case litters
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Ahmed", 28);
    println!("My name is {},and i'm {}",my_name,my_age);
}