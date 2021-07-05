pub fn run() {
    // Print to Console
    println!("Hello From Print.rs File");

    // Basic formatting
    println!("Number: {}", 2);
    println!("{} is here, Iam from {}", "Ahmed", "Egypt");

    //Positional Arguments
    println!("{0} is form {1}, and {0} likes to {2}","Ahmed","Egypt", "code");

    // Named Arguments
    println!("{name} likes to play {active}", name="Ahmed", active="Ninja");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10,10,10);

    //placeholder for debug trait
    println!("{:?}", (12,true,"hello"));

    // basic Math 
    println!("10 + 10 = {}", 10+10);
}